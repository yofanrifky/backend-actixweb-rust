use actix_web::{error, web, HttpMessage, HttpResponse};
use serde::Deserialize;

use crate::{
    auth::{password, role::Role},
    state::AppState,
};

#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub async fn register(
    state: web::Data<AppState>,
    body: web::Json<Credentials>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = body.username.trim();

    if username.is_empty() || body.password.len() < 8 {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Username wajib diisi dan password minimal 8 byte"
        })));
    }

    if body.password.len() > 128 {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Password maksimal 128 byte"
        })));
    }

    let password = body.password.clone();
    let password_hash = web::block(move || password::hash_password(&password))
        .await
        .map_err(|_| error::ErrorInternalServerError("Gagal memproses password"))?
        .map_err(|_| error::ErrorInternalServerError("Gagal mengamankan password"))?;

    match sqlx::query(r#"INSERT INTO "user" (username, password, role) VALUES ($1, $2, $3)"#)
        .bind(username)
        .bind(password_hash)
        .bind(Role::Pembeli.as_str())
        .execute(&state.db)
        .await
    {
        Ok(_) => Ok(HttpResponse::Created().json(serde_json::json!({
            "message": "Akun berhasil dibuat"
        }))),
        Err(sqlx::Error::Database(database_error)) if database_error.is_unique_violation() => {
            Ok(HttpResponse::Conflict().json(serde_json::json!({
                "error": "Username sudah digunakan"
            })))
        }
        Err(_) => Err(error::ErrorInternalServerError("Gagal membuat akun")),
    }
}

pub async fn login(
    state: web::Data<AppState>,
    body: web::Json<Credentials>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = sqlx::query_as::<_, (i32, String, String)>(
        r#"SELECT id, password, role FROM "user" WHERE username = $1"#,
    )
    .bind(body.username.trim())
    .fetch_optional(&state.db)
    .await
    .map_err(|_| error::ErrorInternalServerError("Gagal memeriksa kredensial"))?;

    let Some((user_id, stored_password, role_name)) = user else {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Username atau password salah"
        })));
    };

    let role = Role::from_database(&role_name)
        .ok_or_else(|| error::ErrorInternalServerError("Role akun tidak valid"))?;

    if body.password.len() > 128 {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Username atau password salah"
        })));
    }

    let password = body.password.clone();
    let verification =
        web::block(move || password::verify_and_upgrade(&password, &stored_password))
            .await
            .map_err(|_| error::ErrorInternalServerError("Gagal memproses password"))?
            .map_err(|_| error::ErrorInternalServerError("Gagal mengamankan password"))?;

    if !verification.valid {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Username atau password salah"
        })));
    }

    if let Some(password_hash) = verification.upgraded_hash {
        sqlx::query(r#"UPDATE "user" SET password = $1 WHERE id = $2"#)
            .bind(password_hash)
            .bind(user_id)
            .execute(&state.db)
            .await
            .map_err(|_| error::ErrorInternalServerError("Gagal memperbarui keamanan akun"))?;
    }

    let access_token = state
        .jwt
        .issue(user_id, role)
        .map_err(|_| error::ErrorInternalServerError("Gagal membuat token login"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Login berhasil",
        "access_token": access_token,
        "token_type": "Bearer",
        "expires_in": state.jwt.expiration_seconds(),
        "role": role
    })))
}

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Hapus access_token dari penyimpanan klien"
    }))
}

pub async fn me(
    state: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = request
        .extensions()
        .get::<crate::auth::jwt::AuthenticatedUser>()
        .map(|user| (user.user_id, user.role))
        .ok_or_else(|| error::ErrorUnauthorized("Login diperlukan"))?;
    let (user_id, role) = user_id;

    let username = sqlx::query_scalar::<_, String>(r#"SELECT username FROM "user" WHERE id = $1"#)
        .bind(user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| error::ErrorInternalServerError("Gagal mengambil profil akun"))?;

    match username {
        Some(username) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "id": user_id,
            "username": username,
            "role": role
        }))),
        None => Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Akun tidak lagi tersedia"
        }))),
    }
}

#[derive(Deserialize)]
pub struct RoleInput {
    pub role: String,
}

pub async fn update_user_role(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    body: web::Json<RoleInput>,
) -> Result<HttpResponse, actix_web::Error> {
    let Some(role) = Role::from_database(body.role.trim()) else {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Role harus salah satu dari: admin, kasir, pembeli"
        })));
    };

    let updated = sqlx::query_as::<_, (i32, String, String)>(
        r#"UPDATE "user" SET role = $1 WHERE id = $2 RETURNING id, username, role"#,
    )
    .bind(role.as_str())
    .bind(path.into_inner())
    .fetch_optional(&state.db)
    .await
    .map_err(|_| error::ErrorInternalServerError("Gagal memperbarui role akun"))?;

    match updated {
        Some((id, username, role)) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "id": id,
            "username": username,
            "role": role,
            "message": "Role diperbarui; pengguna harus login kembali"
        }))),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Akun tidak ditemukan"
        }))),
    }
}

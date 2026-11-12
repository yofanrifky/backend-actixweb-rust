use actix_web::{web, HttpResponse, Responder};
use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::handlers::format::format_rupiah;
use crate::{
    models::{MenuInput, MenuItem, MenuPatchInput},
    state::AppState,
};

pub async fn get_menus(
    state: web::Data<AppState>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let field = query.get("field").map(String::as_str);

    match field {
        None => {
            let query = "SELECT id, name, price, category, stock FROM menus ORDER BY id ASC";
            match sqlx::query_as::<_, (i32, String, Decimal, String, i32)>(query)
                .fetch_all(&state.db)
                .await
            {
                Ok(rows) => {
                    let values = rows
                        .into_iter()
                        .map(|(id, name, price, category, stock)| {
                            serde_json::json!([id, name, format_rupiah(price), category, stock])
                        })
                        .collect::<Vec<_>>();
                    HttpResponse::Ok().json(values)
                }
                Err(error) => database_error("Gagal mengambil daftar menu", error),
            }
        }
        Some("id") => match sqlx::query_scalar::<_, i32>("SELECT id FROM menus ORDER BY id ASC")
            .fetch_all(&state.db)
            .await
        {
            Ok(values) => HttpResponse::Ok().json(values),
            Err(error) => database_error("Gagal mengambil daftar menu", error),
        },
        Some("name") => {
            match sqlx::query_scalar::<_, String>("SELECT name FROM menus ORDER BY id ASC")
                .fetch_all(&state.db)
                .await
            {
                Ok(values) => HttpResponse::Ok().json(values),
                Err(error) => database_error("Gagal mengambil daftar menu", error),
            }
        }
        Some("price") => {
            match sqlx::query_scalar::<_, Decimal>("SELECT price FROM menus ORDER BY id ASC")
                .fetch_all(&state.db)
                .await
            {
                Ok(values) => HttpResponse::Ok()
                    .json(values.into_iter().map(format_rupiah).collect::<Vec<_>>()),
                Err(error) => database_error("Gagal mengambil daftar menu", error),
            }
        }
        Some("category") => {
            match sqlx::query_scalar::<_, String>("SELECT category FROM menus ORDER BY id ASC")
                .fetch_all(&state.db)
                .await
            {
                Ok(values) => HttpResponse::Ok().json(values),
                Err(error) => database_error("Gagal mengambil daftar menu", error),
            }
        }
        Some("stock") => {
            match sqlx::query_scalar::<_, i32>("SELECT stock FROM menus ORDER BY id ASC")
                .fetch_all(&state.db)
                .await
            {
                Ok(values) => HttpResponse::Ok().json(values),
                Err(error) => database_error("Gagal mengambil daftar stok", error),
            }
        }
        Some(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Field harus salah satu dari: id, name, price, category, stock"
        })),
    }
}

pub async fn get_menu(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let query = "SELECT id, name, price, category, stock FROM menus WHERE id = $1";

    match sqlx::query_as::<_, MenuItem>(query)
        .bind(path.into_inner())
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(menu)) => HttpResponse::Ok().json(serde_json::json!([
            menu.id,
            menu.name,
            format_rupiah(menu.price),
            menu.category,
            menu.stock
        ])),
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Menu tidak ditemukan"}))
        }
        Err(error) => database_error("Gagal mengambil menu", error),
    }
}

pub async fn create_menu(state: web::Data<AppState>, body: web::Json<MenuInput>) -> impl Responder {
    if let Some(response) = validate_menu(&body) {
        return response;
    }

    let query = "INSERT INTO menus (name, price, category, stock) VALUES ($1, $2, $3, $4) RETURNING id, name, price, category, stock";
    match sqlx::query_as::<_, MenuItem>(query)
        .bind(body.name.trim())
        .bind(body.price)
        .bind(body.category.trim())
        .bind(body.stock)
        .fetch_one(&state.db)
        .await
    {
        Ok(menu) => HttpResponse::Created().json(menu),
        Err(error) => database_error("Gagal membuat menu", error),
    }
}

pub async fn update_menu(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    body: web::Json<MenuInput>,
) -> impl Responder {
    if let Some(response) = validate_menu(&body) {
        return response;
    }

    let query = "UPDATE menus SET name = $1, price = $2, category = $3, stock = $4 WHERE id = $5 RETURNING id, name, price, category, stock";
    match sqlx::query_as::<_, MenuItem>(query)
        .bind(body.name.trim())
        .bind(body.price)
        .bind(body.category.trim())
        .bind(body.stock)
        .bind(path.into_inner())
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(menu)) => HttpResponse::Ok().json(menu),
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Menu tidak ditemukan"}))
        }
        Err(error) => database_error("Gagal memperbarui menu", error),
    }
}

pub async fn patch_menu(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    body: web::Json<MenuPatchInput>,
) -> impl Responder {
    if body.name.is_none()
        && body.price.is_none()
        && body.category.is_none()
        && body.stock.is_none()
    {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Minimal satu field menu harus diisi"
        }));
    }
    if body
        .name
        .as_ref()
        .is_some_and(|name| name.trim().is_empty())
        || body
            .category
            .as_ref()
            .is_some_and(|category| category.trim().is_empty())
    {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Nama dan kategori menu tidak boleh kosong"
        }));
    }
    if body.price.is_some_and(|price| price <= Decimal::ZERO) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Harga harus berupa angka lebih besar dari 0"
        }));
    }
    if body.stock.is_some_and(|stock| stock < 0) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Stok tidak boleh negatif"
        }));
    }

    let query = "UPDATE menus SET name = COALESCE($1, name), price = COALESCE($2, price), category = COALESCE($3, category), stock = COALESCE($4, stock) WHERE id = $5 RETURNING id, name, price, category, stock";
    match sqlx::query_as::<_, MenuItem>(query)
        .bind(body.name.as_deref().map(str::trim))
        .bind(body.price)
        .bind(body.category.as_deref().map(str::trim))
        .bind(body.stock)
        .bind(path.into_inner())
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(menu)) => HttpResponse::Ok().json(menu),
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Menu tidak ditemukan"}))
        }
        Err(error) => database_error("Gagal memperbarui menu", error),
    }
}

pub async fn delete_menu(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    match sqlx::query("DELETE FROM menus WHERE id = $1")
        .bind(path.into_inner())
        .execute(&state.db)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => {
            HttpResponse::Ok().json(serde_json::json!({"message": "Menu berhasil dihapus"}))
        }
        Ok(_) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Menu tidak ditemukan"}))
        }
        Err(error) => database_error("Gagal menghapus menu", error),
    }
}

fn validate_menu(menu: &MenuInput) -> Option<HttpResponse> {
    if menu.name.trim().is_empty() || menu.category.trim().is_empty() {
        return Some(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Nama dan kategori menu wajib diisi"
        })));
    }
    if menu.price <= Decimal::ZERO {
        return Some(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Harga harus berupa angka lebih besar dari 0"
        })));
    }
    if menu.stock < 0 {
        return Some(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Stok tidak boleh negatif"
        })));
    }
    None
}

fn database_error(message: &str, error: sqlx::Error) -> HttpResponse {
    eprintln!("{message}: {error}");
    HttpResponse::InternalServerError().json(serde_json::json!({"error": message}))
}
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorInternalServerError, ErrorUnauthorized},
    http::Method,
    middleware::Next,
    web, Error, HttpMessage,
};

use crate::{
    auth::{jwt::AuthenticatedUser, role::Role},
    state::AppState,
};

pub async fn require_login(
    request: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let (request, _) = authenticate(request).await?;
    next.call(request).await
}

pub async fn require_admin(
    request: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    authorize(request, next, |role| role == Role::Admin).await
}

pub async fn require_menu_access(
    request: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let method = request.method().clone();
    authorize(request, next, move |role| {
        menu_access_allowed(role, &method)
    })
    .await
}

pub async fn require_order_access(
    request: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let method = request.method().clone();
    let path = request.path().to_owned();
    authorize(request, next, move |role| {
        order_access_allowed(role, &method, &path)
    })
    .await
}

pub async fn require_payment_access(
    request: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let method = request.method().clone();
    let path = request.path().to_owned();
    authorize(request, next, move |role| {
        payment_access_allowed(role, &method, &path)
    })
    .await
}

async fn authorize<B, F>(
    request: ServiceRequest,
    next: Next<B>,
    allowed: F,
) -> Result<ServiceResponse<B>, Error>
where
    B: MessageBody + 'static,
    F: FnOnce(Role) -> bool,
{
    let (request, user) = authenticate(request).await?;
    if !allowed(user.role) {
        return Err(actix_web::error::ErrorForbidden(
            "Role tidak memiliki akses ke endpoint ini",
        ));
    }

    next.call(request).await
}

async fn authenticate(
    request: ServiceRequest,
) -> Result<(ServiceRequest, AuthenticatedUser), Error> {
    let token = request
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|value| !value.is_empty())
        .ok_or_else(|| ErrorUnauthorized("Bearer token diperlukan"))?;

    let state = request
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ErrorInternalServerError("Konfigurasi autentikasi tidak tersedia"))?;
    let user = state
        .jwt
        .verify(token)
        .map_err(|_| ErrorUnauthorized("Token tidak valid atau sudah kedaluwarsa"))?;

    let stored_role = sqlx::query_scalar::<_, String>(r#"SELECT role FROM "user" WHERE id = $1"#)
        .bind(user.user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| ErrorInternalServerError("Gagal memvalidasi role akun"))?
        .ok_or_else(|| ErrorUnauthorized("Akun tidak lagi tersedia"))?;
    let current_role = Role::from_database(&stored_role)
        .ok_or_else(|| ErrorInternalServerError("Role akun tidak valid di database"))?;

    if current_role != user.role {
        return Err(ErrorUnauthorized(
            "Role akun berubah; silakan login kembali",
        ));
    }

    request.extensions_mut().insert(user);
    Ok((request, user))
}

fn is_order_detail_path(path: &str) -> bool {
    path.strip_prefix("/api/orders/")
        .is_some_and(|id| !id.is_empty() && id.bytes().all(|byte| byte.is_ascii_digit()))
}

fn menu_access_allowed(role: Role, method: &Method) -> bool {
    match *method {
        Method::GET | Method::HEAD => true,
        Method::POST | Method::PUT | Method::PATCH | Method::DELETE => {
            role == Role::Admin || role == Role::Kasir
        }
        _ => false,
    }
}

fn order_access_allowed(role: Role, method: &Method, path: &str) -> bool {
    role == Role::Admin
        || (role == Role::Pembeli
            && ((method == Method::GET && path == "/api/orders")
                || method == Method::POST
                || (method == Method::GET
                    && (is_order_detail_path(path)
                        || path
                            .strip_suffix("/receipt")
                            .is_some_and(is_order_detail_path)))))
}

fn payment_access_allowed(role: Role, method: &Method, path: &str) -> bool {
    role == Role::Admin
        || (role == Role::Pembeli
            && ((method == Method::POST && path.ends_with("/payment"))
                || (method == Method::GET && path.ends_with("/receipt"))))
}

#[cfg(test)]
mod tests {
    use actix_web::http::Method;

    use crate::auth::role::Role;

    use super::{menu_access_allowed, order_access_allowed, payment_access_allowed};

    #[test]
    fn menu_access_matches_roles() {
        for role in [Role::Admin, Role::Kasir, Role::Pembeli] {
            assert!(menu_access_allowed(role, &Method::GET));
        }
        assert!(menu_access_allowed(Role::Kasir, &Method::PATCH));
        assert!(menu_access_allowed(Role::Admin, &Method::DELETE));
        assert!(!menu_access_allowed(Role::Pembeli, &Method::POST));
    }

    #[test]
    fn order_access_hides_purchase_list_from_cashier_but_allows_buyer_history() {
        assert!(order_access_allowed(
            Role::Admin,
            &Method::GET,
            "/api/orders"
        ));
        assert!(!order_access_allowed(
            Role::Kasir,
            &Method::GET,
            "/api/orders"
        ));
        assert!(order_access_allowed(
            Role::Pembeli,
            &Method::GET,
            "/api/orders"
        ));
        assert!(order_access_allowed(
            Role::Pembeli,
            &Method::POST,
            "/api/orders"
        ));
        assert!(order_access_allowed(
            Role::Pembeli,
            &Method::GET,
            "/api/orders/15"
        ));
        assert!(!order_access_allowed(
            Role::Pembeli,
            &Method::DELETE,
            "/api/orders/15"
        ));
    }

    #[test]
    fn only_admin_or_buyer_can_request_payment_and_receipt() {
        assert!(payment_access_allowed(
            Role::Pembeli,
            &Method::POST,
            "/api/orders/15/payment"
        ));
        assert!(payment_access_allowed(
            Role::Pembeli,
            &Method::GET,
            "/api/orders/15/receipt"
        ));
        assert!(!payment_access_allowed(
            Role::Kasir,
            &Method::GET,
            "/api/orders/15/receipt"
        ));
        assert!(payment_access_allowed(
            Role::Admin,
            &Method::POST,
            "/api/orders/15/payment"
        ));
    }
}

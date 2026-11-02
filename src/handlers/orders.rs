use actix_web::{web, HttpResponse, Responder};
use std::collections::HashMap;

use crate::{
    auth::{jwt::AuthenticatedUser, role::Role},
    models::{CreateOrderInput, Order, OrderInput, OrderPatchInput},
    state::AppState,
};

pub async fn get_orders(
    state: web::Data<AppState>,
    user: web::ReqData<AuthenticatedUser>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let field = query.get("field").map(String::as_str);
    let is_admin = user.role == Role::Admin;
    let result = match field {
        None => {
            let query = "SELECT id, customer_name, item_name, quantity, status FROM orders WHERE $1 OR user_id = $2 ORDER BY id DESC";
            match sqlx::query_as::<_, Order>(query)
                .bind(is_admin)
                .bind(user.user_id)
                .fetch_all(&state.db)
                .await
            {
                Ok(orders) => {
                    let values = orders
                        .into_iter()
                        .map(|order| {
                            serde_json::json!([
                                order.id,
                                order.customer_name,
                                order.item_name,
                                order.quantity,
                                order.status
                            ])
                        })
                        .collect::<Vec<_>>();
                    return HttpResponse::Ok().json(values);
                }
                Err(error) => return database_error("Gagal mengambil daftar pesanan", error),
            }
        }
        Some("id") => sqlx::query_scalar::<_, i32>(
            "SELECT id FROM orders WHERE $1 OR user_id = $2 ORDER BY id DESC",
        )
        .bind(is_admin)
        .bind(user.user_id)
        .fetch_all(&state.db)
        .await
        .map(|values| serde_json::json!(values)),
        Some("customer_name") => sqlx::query_scalar::<_, String>(
            "SELECT customer_name FROM orders WHERE $1 OR user_id = $2 ORDER BY id DESC",
        )
        .bind(is_admin)
        .bind(user.user_id)
        .fetch_all(&state.db)
        .await
        .map(|values| serde_json::json!(values)),
        Some("item_name") => sqlx::query_scalar::<_, String>(
            "SELECT item_name FROM orders WHERE $1 OR user_id = $2 ORDER BY id DESC",
        )
        .bind(is_admin)
        .bind(user.user_id)
        .fetch_all(&state.db)
        .await
        .map(|values| serde_json::json!(values)),
        Some("quantity") => sqlx::query_scalar::<_, i32>(
            "SELECT quantity FROM orders WHERE $1 OR user_id = $2 ORDER BY id DESC",
        )
        .bind(is_admin)
        .bind(user.user_id)
        .fetch_all(&state.db)
        .await
        .map(|values| serde_json::json!(values)),
        Some("status") => sqlx::query_scalar::<_, String>(
            "SELECT status FROM orders WHERE $1 OR user_id = $2 ORDER BY id DESC",
        )
        .bind(is_admin)
        .bind(user.user_id)
        .fetch_all(&state.db)
        .await
        .map(|values| serde_json::json!(values)),
        _ => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Field harus salah satu dari: id, customer_name, item_name, quantity, status"
            }));
        }
    };

    match result {
        Ok(values) => HttpResponse::Ok().json(values),
        Err(error) => database_error("Gagal mengambil daftar pesanan", error),
    }
}

pub async fn get_order(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    user: web::ReqData<AuthenticatedUser>,
) -> impl Responder {
    let order_id = path.into_inner();
    let order = if user.role == Role::Pembeli {
        sqlx::query_as::<_, Order>(
            "SELECT id, customer_name, item_name, quantity, status FROM orders WHERE id = $1 AND user_id = $2",
        )
        .bind(order_id)
        .bind(user.user_id)
        .fetch_optional(&state.db)
        .await
    } else {
        sqlx::query_as::<_, Order>(
            "SELECT id, customer_name, item_name, quantity, status FROM orders WHERE id = $1",
        )
        .bind(order_id)
        .fetch_optional(&state.db)
        .await
    };

    match order {
        Ok(Some(order)) => HttpResponse::Ok().json(serde_json::json!([
            order.id,
            order.customer_name,
            order.item_name,
            order.quantity,
            order.status
        ])),
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Pesanan tidak ditemukan"}))
        }
        Err(error) => database_error("Gagal mengambil pesanan", error),
    }
}

pub async fn create_order(
    state: web::Data<AppState>,
    user: web::ReqData<AuthenticatedUser>,
    body: web::Json<CreateOrderInput>,
) -> impl Responder {
    if let Some(response) = validate_new_order(&body) {
        return response;
    }

    let mut transaction = match state.db.begin().await {
        Ok(transaction) => transaction,
        Err(error) => return database_error("Gagal memulai pembelian", error),
    };

    let (item_name, unit_price) = match sqlx::query_as::<_, (String, rust_decimal::Decimal)>(
        "UPDATE menus SET stock = stock - $1 WHERE id = $2 AND stock >= $1 RETURNING name, price",
    )
    .bind(body.quantity)
    .bind(body.menu_id)
    .fetch_optional(&mut *transaction)
    .await
    {
        Ok(Some(item_name)) => item_name,
        Ok(None) => {
            let _ = transaction.rollback().await;
            return HttpResponse::Conflict().json(serde_json::json!({
                "error": "Menu tidak ditemukan atau stok tidak mencukupi"
            }));
        }
        Err(error) => {
            let _ = transaction.rollback().await;
            return database_error("Gagal memeriksa dan mengurangi stok", error);
        }
    };

    let order = match sqlx::query_as::<_, Order>(
        "INSERT INTO orders (customer_name, item_name, quantity, status, user_id, menu_id, unit_price) VALUES ($1, $2, $3, 'pending', $4, $5, $6) RETURNING id, customer_name, item_name, quantity, status",
    )
    .bind(body.customer_name.trim())
    .bind(item_name)
    .bind(body.quantity)
    .bind(user.user_id)
    .bind(body.menu_id)
    .bind(unit_price)
    .fetch_one(&mut *transaction)
    .await
    {
        Ok(order) => order,
        Err(error) => {
            let _ = transaction.rollback().await;
            return database_error("Gagal membuat pesanan", error);
        }
    };

    if let Err(error) = transaction.commit().await {
        return database_error("Gagal menyelesaikan pembelian", error);
    }

    HttpResponse::Created().json(order)
}

pub async fn update_order(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    body: web::Json<OrderInput>,
) -> impl Responder {
    if let Some(response) = validate_order(&body) {
        return response;
    }

    let query = "UPDATE orders SET customer_name = $1, item_name = $2, quantity = $3, status = $4 WHERE id = $5 RETURNING id, customer_name, item_name, quantity, status";
    match sqlx::query_as::<_, Order>(query)
        .bind(body.customer_name.trim())
        .bind(body.item_name.trim())
        .bind(body.quantity)
        .bind(body.status.trim())
        .bind(path.into_inner())
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(order)) => HttpResponse::Ok().json(order),
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Pesanan tidak ditemukan"}))
        }
        Err(error) => database_error("Gagal memperbarui pesanan", error),
    }
}

pub async fn patch_order(
    state: web::Data<AppState>,
    path: web::Path<i32>,
    body: web::Json<OrderPatchInput>,
) -> impl Responder {
    if body.customer_name.is_none()
        && body.item_name.is_none()
        && body.quantity.is_none()
        && body.status.is_none()
    {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Minimal satu field pesanan harus diisi"
        }));
    }
    if body
        .customer_name
        .as_ref()
        .is_some_and(|name| name.trim().is_empty())
        || body
            .item_name
            .as_ref()
            .is_some_and(|name| name.trim().is_empty())
        || body
            .status
            .as_ref()
            .is_some_and(|status| status.trim().is_empty())
    {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Nama pelanggan, nama menu, dan status tidak boleh kosong"
        }));
    }
    if body.quantity.is_some_and(|quantity| quantity <= 0) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Jumlah pesanan harus lebih besar dari 0"
        }));
    }
    if body.status.as_deref() == Some("paid") {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Status paid hanya dapat ditetapkan melalui konfirmasi pembayaran"
        }));
    }

    let query = "UPDATE orders SET customer_name = COALESCE($1, customer_name), item_name = COALESCE($2, item_name), quantity = COALESCE($3, quantity), status = COALESCE($4, status) WHERE id = $5 RETURNING id, customer_name, item_name, quantity, status";
    match sqlx::query_as::<_, Order>(query)
        .bind(body.customer_name.as_deref().map(str::trim))
        .bind(body.item_name.as_deref().map(str::trim))
        .bind(body.quantity)
        .bind(body.status.as_deref().map(str::trim))
        .bind(path.into_inner())
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(order)) => HttpResponse::Ok().json(order),
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Pesanan tidak ditemukan"}))
        }
        Err(error) => database_error("Gagal memperbarui pesanan", error),
    }
}

pub async fn delete_order(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    match sqlx::query("DELETE FROM orders WHERE id = $1")
        .bind(path.into_inner())
        .execute(&state.db)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => {
            HttpResponse::Ok().json(serde_json::json!({"message": "Pesanan berhasil dihapus"}))
        }
        Ok(_) => {
            HttpResponse::NotFound().json(serde_json::json!({"error": "Pesanan tidak ditemukan"}))
        }
        Err(error) => database_error("Gagal menghapus pesanan", error),
    }
}

fn validate_order(order: &OrderInput) -> Option<HttpResponse> {
    if order.customer_name.trim().is_empty()
        || order.item_name.trim().is_empty()
        || order.status.trim().is_empty()
    {
        return Some(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Nama pelanggan, nama menu, dan status wajib diisi"
        })));
    }
    if order.quantity <= 0 {
        return Some(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Jumlah pesanan harus lebih besar dari 0"
        })));
    }
    if order.status == "paid" {
        return Some(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Status paid hanya dapat ditetapkan melalui konfirmasi pembayaran"
        })));
    }
    None
}

fn validate_new_order(order: &CreateOrderInput) -> Option<HttpResponse> {
    if order.customer_name.trim().is_empty() {
        return Some(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Nama pelanggan wajib diisi"
        })));
    }
    if order.menu_id <= 0 || order.quantity <= 0 {
        return Some(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "ID menu dan jumlah pesanan harus lebih besar dari 0"
        })));
    }
    None
}

fn database_error(message: &str, error: sqlx::Error) -> HttpResponse {
    eprintln!("{message}: {error}");
    HttpResponse::InternalServerError().json(serde_json::json!({"error": message}))
}

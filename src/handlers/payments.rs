use actix_web::{error, web, HttpResponse};
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{auth::jwt::AuthenticatedUser, state::AppState};

#[derive(Deserialize)]
pub struct CreatePaymentInput {
    pub method: String,
}

pub async fn list_payments(state: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    let payments = sqlx::query_as::<_, (i32, i32, Option<i32>, String, String, String)>(
        r#"SELECT p.id, p.order_id, p.user_id, p.method, p.status,
                  to_char(p.created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"')
           FROM payments p ORDER BY p.id DESC"#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| error::ErrorInternalServerError("Gagal mengambil daftar pembayaran"))?;

    let items = payments
        .into_iter()
        .map(|(id, order_id, user_id, method, status, created_at)| {
            serde_json::json!({
                "id": id,
                "order_id": order_id,
                "user_id": user_id,
                "method": method,
                "status": status,
                "created_at": created_at
            })
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(items))
}

pub async fn create_payment(
    state: web::Data<AppState>,
    user: web::ReqData<AuthenticatedUser>,
    path: web::Path<i32>,
    body: web::Json<CreatePaymentInput>,
) -> Result<HttpResponse, actix_web::Error> {
    let method = body.method.trim();
    if !matches!(method, "cash" | "bank_transfer") {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Metode pembayaran harus cash atau bank_transfer"
        })));
    }

    let order_id = path.into_inner();
    let payment = sqlx::query_as::<_, (i32, String, String)>(
        r#"INSERT INTO payments (order_id, user_id, method)
           SELECT id, user_id, $3 FROM orders
           WHERE id = $1 AND ($2 OR user_id = $4)
           RETURNING id, method, status"#,
    )
    .bind(order_id)
    .bind(user.role == crate::auth::role::Role::Admin)
    .bind(method)
    .bind(user.user_id)
    .fetch_optional(&state.db)
    .await;

    match payment {
        Ok(Some((payment_id, method, status))) => {
            Ok(HttpResponse::Created().json(serde_json::json!({
                "payment_id": payment_id,
                "order_id": order_id,
                "method": method,
                "status": status,
                "message": "Pembayaran menunggu verifikasi admin"
            })))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Pesanan tidak ditemukan atau bukan milik akun ini"
        }))),
        Err(sqlx::Error::Database(database_error)) if database_error.is_unique_violation() => {
            Ok(HttpResponse::Conflict().json(serde_json::json!({
                "error": "Pembayaran untuk pesanan ini sudah dibuat"
            })))
        }
        Err(_) => Err(error::ErrorInternalServerError(
            "Gagal membuat permintaan pembayaran",
        )),
    }
}

pub async fn confirm_payment(
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut transaction = state
        .db
        .begin()
        .await
        .map_err(|_| error::ErrorInternalServerError("Gagal memulai konfirmasi pembayaran"))?;
    let payment = sqlx::query_as::<_, (i32, i32, String)>(
        "UPDATE payments SET status = 'paid', paid_at = NOW() WHERE id = $1 AND status = 'pending' RETURNING id, order_id, status",
    )
    .bind(path.into_inner())
    .fetch_optional(&mut *transaction)
    .await
    .map_err(|_| error::ErrorInternalServerError("Gagal mengonfirmasi pembayaran"))?;

    match payment {
        Some((payment_id, order_id, status)) => {
            sqlx::query("UPDATE orders SET status = 'paid' WHERE id = $1")
                .bind(order_id)
                .execute(&mut *transaction)
                .await
                .map_err(|_| error::ErrorInternalServerError("Gagal memperbarui status pesanan"))?;
            transaction.commit().await.map_err(|_| {
                error::ErrorInternalServerError("Gagal menyimpan konfirmasi pembayaran")
            })?;

            Ok(HttpResponse::Ok().json(serde_json::json!({
                "payment_id": payment_id,
                "order_id": order_id,
                "status": status,
                "message": "Pembayaran dikonfirmasi"
            })))
        }
        None => {
            transaction.rollback().await.map_err(|_| {
                error::ErrorInternalServerError("Gagal membatalkan konfirmasi pembayaran")
            })?;
            Ok(HttpResponse::Conflict().json(serde_json::json!({
                "error": "Pembayaran tidak ditemukan atau bukan berstatus pending"
            })))
        }
    }
}

pub async fn get_receipt(
    state: web::Data<AppState>,
    user: web::ReqData<AuthenticatedUser>,
    path: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let receipt = sqlx::query_as::<_, (i32, String, String, i32, Option<Decimal>, Option<String>)>(
        r#"SELECT o.id, o.customer_name, o.item_name, o.quantity,
                  o.unit_price * o.quantity, to_char(p.paid_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"')
           FROM orders o
           JOIN payments p ON p.order_id = o.id
           WHERE o.id = $1 AND p.status = 'paid' AND ($2 OR o.user_id = $3)"#,
    )
    .bind(path.into_inner())
    .bind(user.role == crate::auth::role::Role::Admin)
    .bind(user.user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| error::ErrorInternalServerError("Gagal mengambil struk"))?;

    match receipt {
        Some((order_id, customer_name, item_name, quantity, total, paid_at)) => {
            let Some(total) = total else {
                return Ok(HttpResponse::Conflict().json(serde_json::json!({
                    "error": "Struk tidak tersedia untuk pesanan lama tanpa snapshot harga"
                })));
            };
            let Some(paid_at) = paid_at else {
                return Ok(HttpResponse::Conflict().json(serde_json::json!({
                    "error": "Waktu pembayaran tidak tersedia untuk struk ini"
                })));
            };
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "receipt_number": format!("RST-{order_id}"),
                "order_id": order_id,
                "customer_name": customer_name,
                "item_name": item_name,
                "quantity": quantity,
                "total": total,
                "currency": "IDR",
                "paid_at": paid_at
            })))
        }
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Struk tidak ditemukan, belum dibayar, atau bukan milik akun ini"
        }))),
    }
}

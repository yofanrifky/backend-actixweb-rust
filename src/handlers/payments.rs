use crate::{auth::jwt::AuthenticatedUser, state::AppState};
use actix_web::{error, web, HttpResponse};
use rust_decimal::Decimal;

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
            WHERE o.id = $1 AND p.status = 'lunas' AND ($2 OR o.user_id = $3)"#,
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

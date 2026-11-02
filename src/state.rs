use crate::auth::jwt::JwtService;
use sqlx::PgPool;

pub struct AppState {
    pub db: PgPool,
    pub jwt: JwtService,
}

use sqlx::postgres::PgPool;
use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub sign_count: Mutex<u32>,
    pub db: PgPool,
}

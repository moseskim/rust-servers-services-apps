
// Listing 8.x - 시작

use sqlx::postgres::PgPool;

pub struct AppState {
    pub db: PgPool,
}


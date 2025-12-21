use sqlx::SqlitePool;

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

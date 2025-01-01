use sqlx::{PgPool, Pool, Postgres, Error};

pub async fn init_pool(database_url: &str) -> Result<Pool<Postgres>, Error> {
    PgPool::connect(database_url).await
}

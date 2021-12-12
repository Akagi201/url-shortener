use sqlx::postgres::PgPoolOptions;
use sqlx::{Postgres, Pool};

pub async fn do_connect() -> Pool<Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root@127.0.0.1:5432/url-shortener").await;
    pool.unwrap()
}

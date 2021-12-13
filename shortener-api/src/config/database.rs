use sqlx::postgres::PgPoolOptions;
use sqlx::{Postgres, Pool};
use redis::aio::Connection;

pub async fn do_connect() -> Pool<Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root@127.0.0.1:5432/url-shortener").await;
    pool.unwrap()
}

pub async fn do_redis_connect() -> Connection {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    client.get_async_connection().await.unwrap()
}

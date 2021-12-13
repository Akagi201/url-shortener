use sqlx::{Error, Postgres, Pool, FromRow};
use sqlx::postgres::PgQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct ShortLink {
    pub id: u32,
    pub url: String,
}

pub async fn create_shortlink(pool: &Pool<Postgres>, url: &str) -> Result<PgQueryResult, Error> {
    sqlx::query(
        r#"
            INSERT INTO short_links (`url`)
            VALUES(?)"#,
    )
        .bind(url)
        .execute(pool).await
}

pub async fn delete_shortlink(pool: &Pool<Postgres>, id: i32) -> Result<PgQueryResult, Error> {
    sqlx::query(
        r#"
            DELETE FROM short_links
            WHERE id = ?
            "#,
    )
        .bind(id)
        .execute(pool).await
}

#[allow(dead_code)]
pub async fn get_shortlink(pool: &Pool<Postgres>, id: i32) -> Result<ShortLink, Error> {
    sqlx::query_as::<_, ShortLink>(
        r#"
            SELECT * FROM short_links
            WHERE id = ?
            "#,
    )
        .bind(id)
        .fetch_one(pool).await
}

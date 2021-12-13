// use sqlx::{Pool, MySql};
// use redis::aio::Connection;
// use std::sync::Arc;
//
// #[derive(Clone, Debug)]
// pub struct Environment {
//     postgres_conn: Pool<Postgres>,
//     redis_conn: Arc::new<Connection>
// }
//
// impl Environment {
//     pub async fn new(postgres_conn: Pool<Postgres>, redis_conn: Arc::new<Connection>) -> anyhow::Result<Self> {
//         Ok(Self {
//             postgres_conn,
//             redis_conn
//         })
//     }
//
//     pub fn db(self) -> Pool<Postgres> {
//         self.postgres_conn
//     }
//
//     pub fn clients(self) -> Arc::new<Connection> {
//         self.redis_conn
//     }
// }
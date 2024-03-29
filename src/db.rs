use std::env;

use sqlx::{Pool, Sqlite, SqlitePool};
use time::PrimitiveDateTime;

pub async fn establish_connection() -> Pool<Sqlite> {
    // 从环境变量读取数据库连接信息
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL cannot be empty");
    // 连接数据库
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("can't connect to database");
    pool
}

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub openid: String,
    pub session_key: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}
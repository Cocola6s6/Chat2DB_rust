use crate::models::db::Db;
use actix_web::{web, HttpResponse, Responder};

// sql_handler
pub async fn sql_handler(db_url: &str, sql: &str, code: u32) -> impl Responder {
    let _ = Db::exec_sql(db_url, sql, code);
    HttpResponse::Ok().json("sql_handler")
}

// table_handler
pub async fn table_handler(db_url: &str, db_ns: &str) -> impl Responder {
    let resp = Db::query_tables(db_url, db_ns).await.unwrap();
    HttpResponse::Ok().json(resp)
}

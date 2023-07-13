use crate::models::chat::Chat;
use crate::models::sql::Sql;
use actix_web::{web, HttpResponse, Responder};

// hello_handler
pub async fn hello_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello")
}

// chatgpt_handler
pub async fn chatgpt_handler(openai_key: &str, url: &str, ns: &str, text: &str) -> impl Responder {
    let response = Chat::execute_chat(openai_key, url, ns, text).await.unwrap();
    HttpResponse::Ok().json(response)
}

// chatgpt_handler
pub async fn sql_handler(url: &str, sql: &str, code: u32) -> impl Responder {
    Sql::execute_sql(url, sql, code);
    HttpResponse::Ok().json("sql_handler")
}

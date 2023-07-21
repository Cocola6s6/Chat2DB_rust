use anyhow::Result;
use crate::models::chat::Chat;
use actix_web::{HttpResponse, Responder};

// hello_handler
pub async fn hello_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello")
}

// chatgpt_handler
pub async fn chat_handler(openai_key: &str, db_url: &str, db_ns: &str, text: &str) -> Result<HttpResponse> {
    let resp = Chat::exec_chat(openai_key, db_url, db_ns, text).await?;
    Ok(HttpResponse::Ok().json(resp))
}

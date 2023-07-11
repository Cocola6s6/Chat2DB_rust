use crate::models::chat::Chat;
use actix_web::{web, HttpResponse, Responder};

// hello_handler
pub async fn hello_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello")
}

// chatgpt_handler
pub async fn chatgpt_handler(chat: Chat) -> impl Responder {
    println!("Received new course");

    chat.do_chat(&chat.sql).await.unwrap();
    HttpResponse::Ok().json("")
}

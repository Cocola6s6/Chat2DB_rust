use crate::handlers::chatgpt::chatgpt_handler;
use crate::handlers::chatgpt::hello_handler;
use crate::models::chat::Chat;
use actix_web::{web, get, Responder, HttpResponse, post};

// router2
// curl -X POST localhost:5000/courses/insert -H "Content-Type: application/json" -d '{"id": 4, "teacher_id": 1, "course_name": "First course"}'
// curl -X GET localhost:5000/courses/1
// curl -X GET localhost:5000/courses/1/1
// curl -X GET localhost:5000/courses/
// curl -X GET localhost:5000/courses/delete/1
// pub fn chatgpt_router(cfg: &mut web::ServiceConfig) {
//     cfg.service(web::scope("/chat").route("/chatgpt", web::post().to(chatgpt_handler)));
// }


#[get("/chat/hello")]
async fn hello() -> impl Responder {
    println!("[hello]=========================>");
    hello_handler().await
}

#[post("/chat/chatgpt")]
async fn chatgpt_router(req_body: String) -> impl Responder {
    println!("[chatgpt_router]=========================>{:?}", req_body);
    let chat: Chat = serde_json::from_str::<Chat>(&req_body).unwrap();
    let openai_key = chat.openai_key.clone();
    let url = chat.sql.url.clone();
    let ns = chat.sql.ns.clone();
    let text = chat.text.clone();
    chatgpt_handler(&openai_key, &url, &ns, &text).await
}

#[post("/chat/sql")]
async fn sql_router(req_body: String) -> impl Responder {
    println!("[sql_router]=========================>{:?}", req_body);
    hello_handler().await
}
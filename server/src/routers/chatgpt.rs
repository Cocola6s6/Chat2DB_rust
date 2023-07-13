use crate::handlers::chatgpt::chatgpt_handler;
use crate::handlers::chatgpt::hello_handler;
use crate::handlers::chatgpt::sql_handler;
use crate::handlers::chatgpt::table_handler;
use crate::models::chat_req::ChatReq;
use crate::models::table_req::TableReq;
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
async fn chatgpt_router(req: web::Json<ChatReq>) -> impl Responder {
    println!("[chatgpt_router]=========================>{:?}", req);
    chatgpt_handler(&req.openai_key, &req.sql.url, &req.sql.ns, &req.text).await
}

#[get("/chat/sql")]
async fn sql_router(req_body: String) -> impl Responder {
    println!("[sql_router]=========================>{:?}", req_body);
    hello_handler().await
}

// curl -X GET localhost:5000/chat/table?ns=public&url=postgres://postgres:postgres@45.128.222.100:15432
#[get("/chat/table")]
async fn table_router(req: web::Query<TableReq>) -> impl Responder {
    println!("[table_router]=========================>{:?}", req);
    table_handler(&req.url, &req.ns).await
}
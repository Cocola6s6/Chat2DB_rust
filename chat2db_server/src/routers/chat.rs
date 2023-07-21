use crate::handlers::chat::chat_handler;
use crate::handlers::chat::hello_handler;
use crate::models::chat_req::ChatReq;
use actix_web::{get, post, web, HttpResponse, Responder};

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

#[post("/chat/execChat")]
async fn chat_router(req: web::Json<ChatReq>) -> HttpResponse {
    println!("[chat_router]=========================>{:?}", req);
    let resp = chat_handler(&req.openai_key, &req.sql.db_url, &req.sql.db_ns, &req.text).await;

    match resp {
        Ok(resp) => resp,
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

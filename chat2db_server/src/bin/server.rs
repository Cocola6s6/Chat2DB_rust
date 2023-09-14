use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use std::io;
use chat2db_server::routers::chat::chat_router;
use chat2db_server::routers::chat::hello;
use chat2db_server::routers::db::sql_router;
use chat2db_server::routers::db::table_router;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let app = move || {
        // 跨域配置
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8080")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://127.0.0.1")
            })
            .allowed_methods(vec!["GET", "POST", "DELETE", "UPDATE"])
            .allowed_headers(vec![http::header::ACCEPT, http::header::CONTENT_TYPE])
            .max_age(3600);

        App::new().wrap(cors).service(chat_router).service(hello).service(sql_router).service(table_router)
    };

    // 启动
    HttpServer::new(app).bind("localhost:5000")?.run().await
}

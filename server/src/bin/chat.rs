use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use server::models::sql;
use std::io;
use server::routers::chatgpt::chatgpt_router;
use server::routers::chatgpt::hello;
use server::routers::chatgpt::sql_router;
use server::routers::chatgpt::table_router;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let app = move || {
        // 跨域配置
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080/")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST", "DELETE", "UPDATE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new().wrap(cors).service(chatgpt_router).service(hello).service(sql_router).service(table_router)
    };

    // 启动
    HttpServer::new(app).bind("localhost:5000")?.run().await
}

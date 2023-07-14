use crate::handlers::db::sql_handler;
use crate::handlers::db::table_handler;
use crate::models::table_req::TableReq;
use crate::models::sql_req::SqlReq;
use actix_web::{web, get, Responder, post};


#[post("/db/execSql")]
async fn sql_router(req: web::Json<SqlReq>) -> impl Responder {
    println!("[sql_router]=========================>{:?}", req);
    sql_handler(&req.db_url, &req.sql, 2).await
}

// curl -X GET localhost:5000/chat/table?ns=public&url=postgres://postgres:postgres@45.128.222.100:15432
#[get("/db/queryTables")]
async fn table_router(req: web::Query<TableReq>) -> impl Responder {
    println!("[table_router]=========================>{:?}", req);
    table_handler(&req.db_url, &req.db_ns).await
}
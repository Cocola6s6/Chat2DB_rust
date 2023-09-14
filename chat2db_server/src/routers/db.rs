use crate::handlers::db::sql_handler;
use crate::handlers::db::table_handler;
use crate::models::sql_req::SqlReq;
use crate::models::tables_req::TablesReq;
use actix_web::{get, post, web, HttpResponse};

// curl -X POST localhost:5000/db/execSql -H "Content-Type: application/json" -d '{"db_url":"postgres://postgres:postgres@45.128.222.100:15432","sql":"select * from public.test"}'
#[post("/db/execSql")]
async fn sql_router(req: web::Json<SqlReq>) -> HttpResponse {
    println!("[sql_router]=========================>{:?}", req);
    let resp = sql_handler(&req.db_url, &req.sql, 2).await;

    match resp {
        Ok(resp) => resp,
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

// curl -X GET localhost:5000/db/queryTables?db_ns=public&db_url=postgres://postgres:postgres@45.128.222.100:15432
#[get("/db/queryTables")]
async fn table_router(req: web::Query<TablesReq>) -> HttpResponse {
    println!("[table_router]=========================>{:?}", req);
    let resp = table_handler(&req.db_url, &req.db_ns).await;

    println!("{:?}", HttpResponse::Ok().json("value"));
    match resp {
        Ok(resp) => resp,
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

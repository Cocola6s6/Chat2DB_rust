use crate::handlers::connection::{conn_handller, dis_conn_handler, get_conn_handler};
use crate::models::connection::{ConnReq, DisConnReq, GetConnReq};
use actix_web::{post, get, web, HttpResponse};

// curl -X POST localhost:5000/conn/conn -H "Content-Type: application/json" -d '{"openai_key":"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", "db_url":"postgres://postgres:postgres@45.128.222.100:15432", "db_ns":"public"}'
#[post("/conn/conn")]
async fn conn_router(req: web::Json<ConnReq>) -> HttpResponse {
    println!("[conn_router]=========================>{:?}", req);
    let req = ConnReq {
        openai_key: req.openai_key.clone(),
        db_url: req.db_url.clone(),
        db_ns: req.db_ns.clone(),
    };
    let resp = conn_handller(req).await;

    match resp {
        Ok(resp) => resp,
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

// curl -X POST localhost:5000/conn/dis_conn -H "Content-Type: application/json" -d '{"console_id":1}'
#[post("/conn/dis_conn")]
async fn dis_conn_router(req: web::Json<DisConnReq>) -> HttpResponse {
    println!("[dis_conn_router]=========================>{:?}", req);
    let req = DisConnReq {
        console_id: req.console_id.clone(),
    };
    let resp = dis_conn_handler(req).await;

    match resp {
        Ok(resp) => resp,
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

// curl -X GET localhost:5000/conn/get_conn?console_id=1
#[get("/conn/get_conn")]
async fn get_conn_router(req: web::Query<GetConnReq>) -> HttpResponse {
    println!("[get_conn_router]=========================>{:?}", req);
    let req = GetConnReq {
        console_id: req.console_id.clone(),
    };
    let resp = get_conn_handler(req).await;

    match resp {
        Ok(resp) => resp,
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
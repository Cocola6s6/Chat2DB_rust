use anyhow::Result;
use actix_web::{HttpResponse};

use crate::models::connection::{Connection, ConnReq, DisConnReq, GetConnReq};

// connection_handler
pub async fn conn_handller(req: ConnReq) -> Result<HttpResponse> {
    let resp = Connection::connect(req).await?;
    Ok(HttpResponse::Ok().json(resp))
}

// dis_conn_handler
pub async fn dis_conn_handler(req: DisConnReq) -> Result<HttpResponse> {
    let resp = Connection::dis_conn(req).await?;
    Ok(HttpResponse::Ok().json(resp))
}

// get_conn_handler
pub async fn get_conn_handler(req: GetConnReq) -> Result<HttpResponse> {
    let resp = Connection::get_conn(req).await?;
    Ok(HttpResponse::Ok().json(resp))
}

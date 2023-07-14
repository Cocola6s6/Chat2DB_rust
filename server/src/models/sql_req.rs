use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SqlReq {
    pub db_url: String,
    pub sql: String,
}


use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TableReq {
    pub db_url: String,
    pub db_ns: String,
}


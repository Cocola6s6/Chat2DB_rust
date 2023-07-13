use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TableReq {
    pub url: String,
    pub ns: String,
}


use crate::models::db::Db;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatReq {
    pub openai_key: String,
    pub sql: Db,
    pub text: String,
}


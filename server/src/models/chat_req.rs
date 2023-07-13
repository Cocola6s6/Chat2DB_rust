use crate::models::sql::Sql;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatReq {
    pub openai_key: String,
    pub sql: Sql,
    pub text: String,
}


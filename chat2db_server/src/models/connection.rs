use crate::{common::cache_utils::CacheUtils, handlers::db, models::db::Db};
use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnReq {
    pub openai_key: String,
    pub db_url: String,
    pub db_ns: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisConnReq {
    pub console_id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetConnReq {
    pub console_id: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connection {
    pub console_id: usize,
    pub openai_key: String,
    pub db: Db,
}

impl Connection {
    pub async fn connect(req: ConnReq) -> Result<usize> {
        let console_id = 1;
        let connection = Connection {
            console_id: console_id,
            openai_key: req.openai_key,
            db: Db {
                db_url: req.db_url,
                db_ns: req.db_ns,
            },
        };
        let _ = CacheUtils::add_cache(console_id, connection).await;

        Ok(console_id)
    }

    pub async fn dis_conn(req: DisConnReq) -> Result<()> {
        let _ = CacheUtils::rm_cache(req.console_id);

        Ok(())
    }

    pub async fn get_conn(req: GetConnReq) -> Result<Connection> {
        let connection = CacheUtils::get_cache(req.console_id).await;

        connection
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use tokio::test;

    #[test]
    async fn test_conn() {
        dotenv().unwrap();
        let openai_key = std::env::var("OPENAI_KEY").unwrap_or_else(|_| "".to_string());
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "".to_string());
        let db_ns = std::env::var("DATABASE_NS").unwrap_or_else(|_| "".to_string());
        let req = ConnReq {
            openai_key,
            db_url,
            db_ns,
        };

        let resp = Connection::connect(req).await.unwrap();
        println!("resp={:#?}", resp);
    }

    #[test]
    async fn test_dis_conn() {
        let console_id = 1;
        let req = DisConnReq { console_id };
        let resp = Connection::dis_conn(req).await.unwrap();
        println!("resp={:#?}", resp);
    }

    #[test]
    async fn get_conn() {
        let console_id = 1;
        let req = GetConnReq { console_id };
        let resp = Connection::get_conn(req).await.unwrap();
        println!("resp={:#?}", resp);
    }
}

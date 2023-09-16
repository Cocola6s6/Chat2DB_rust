use std::collections::{HashMap, BTreeMap};

use crate::common::{content, http_utils::HttpUtils};
use anyhow::{Ok, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Db {
    pub db_url: String,
    pub db_ns: String,
}

impl Db {
    pub async fn exec_sql(db_url: String, sql: String) -> Result<Vec<BTreeMap<String, String>>> {
        info!("[exec_sql]======================>{:?},{:?}", db_url, sql);

        let url = content::exec_sql_url.to_owned();
        let body = format!(
            r#"
        {{
            "db_url": "{}",
            "sql": "{}"
        }}
        "#,
            db_url, sql
        );
        let resp = HttpUtils::post(url, None, body).await?;
        let resp: Vec<BTreeMap<String, String>> = resp.into_serde().unwrap();
        info!("resp: {:?}", resp);

        Ok(resp)
    }

    pub async fn query_tables(db_url: String, db_ns: String) -> Result<Vec<String>> {
        info!(
            "[query_tables]======================>{:?},{:?}",
            db_url, db_ns
        );

        let url = content::query_tables_url;
        let url = format!("{}?db_url={}&db_ns={}", url, db_url, db_ns);
        let resp = HttpUtils::get(url, None).await?;
        let resp = resp.into_serde().unwrap();
        info!("resp: {:?}", resp);

        Ok(resp)
    }
}

// override default values with environment variables
impl Default for Db {
    fn default() -> Db {
        Db {
            db_url: "postgres://postgres:postgres@45.128.222.100:15432".to_string(),
            db_ns: "public".to_string(),
        }
    }
}

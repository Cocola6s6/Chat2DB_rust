use std::collections::{BTreeMap, HashMap};

use crate::common::{content, http_utils::HttpUtils};
use anyhow::{Ok, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::info;

use super::db::Db;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Connection {
    pub console_id: usize,
    pub openai_key: String,
    pub db: Db,
}

impl Connection {
    pub async fn conn(openai_key: String, db_url: String, db_ns: String) -> Result<usize> {
        info!(
            "[conn]======================>{:?},{:?},{:?}",
            openai_key, db_url, db_ns
        );

        let url = content::conn_url.to_owned();
        let body = format!(
            r#"
        {{
            "openai_key": "{}",
            "db_url": "{}",
            "db_ns": "{}"
        }}
        "#,
            openai_key, db_url, db_ns
        );
        let resp = HttpUtils::post(url, None, body).await?;
        let resp: usize = resp.into_serde().unwrap();
        info!("resp: {:?}", resp);

        Ok(resp)
    }

    pub async fn dis_conn(console_id: usize) -> Result<()> {
        info!("[dis_conn]======================>{:?}", console_id);

        let url = content::dis_conn_url.to_owned();
        let body = format!(
            r#"
        {{
            "console_id": "{}"
        }}
        "#,
            console_id
        );
        let resp = HttpUtils::post(url, None, body).await?;
        let resp = resp.into_serde().unwrap();
        info!("resp: {:?}", resp);

        Ok(resp)
    }

    pub async fn get_conn(console_id: usize) -> Result<Connection> {
        info!("[get_conn]======================>{:?}", console_id);

        let url = content::get_conn_url;
        let url = format!("{}?console_id={}", url, console_id);
        let resp = HttpUtils::get(url, None).await?;
        let resp = resp.into_serde().unwrap();
        info!("resp: {:?}", resp);

        Ok(resp)
    }
}

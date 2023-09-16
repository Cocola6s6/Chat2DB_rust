use crate::common::{content, http_utils::HttpUtils};
use anyhow::{Ok, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Chat {
    pub openai_key: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ChatResp {
    pub code: u8,
    pub result: String,
}

impl Chat {
    pub async fn exec_chat(
        openai_key: String,
        db_url: String,
        db_ns: String,
        text: String,
    ) -> Result<String> {
        info!(
            "[exec_chat]======================>{:?}, {:?}, {:?}, {:?}",
            openai_key, db_url, db_ns, text
        );

        let body = format!(
            r#"
        {{
            "openai_key": "{}",
            "sql": {{
                "db_url": "{}",
                "db_ns": "{}"
            }},
            "text": "{}"
        }}
        "#,
            openai_key, db_url, db_ns, text
        );

        let resp = HttpUtils::post(content::exec_chat_url.to_owned(), None, body).await?;
        let resp: String = resp.into_serde().unwrap();
        info!("resp: {:?}", resp);
        let resp: ChatResp = serde_json::from_str(&resp)?;

        Ok(resp.result)
    }
}

// override default values with environment variables
impl Default for Chat {
    fn default() -> Self {
        Chat {
            openai_key: "sk-34a5ce02952b436bb955dab064177c20".to_string(),
        }
    }
}

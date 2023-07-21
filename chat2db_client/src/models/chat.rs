use crate::common::{content, http_utils::HttpUtils};
use anyhow::{Ok, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Chat {
    pub openai_key: String,
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
        let resp = resp.into_serde().unwrap();
        info!("resp: {:?}", resp);

        Ok(resp)
    }
}

// override default values with environment variables
impl Default for Chat {
    fn default() -> Self {
        Chat {
            openai_key: "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".to_string(),
        }
    }
}

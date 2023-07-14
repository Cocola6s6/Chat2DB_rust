use crate::common::content;
use anyhow::{Ok, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::info;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Chat {
    pub openai_key: String,
}
// TODO 将http请求抽象出来
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
        // 1、创建Request请求
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.mode(RequestMode::Cors);

        let headers = Headers::new().unwrap();
        headers.set("Accept", "application/json");
        headers.set("Content-Type", "application/json");
        opts.headers(&headers);

        let str_json = format!(
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
        opts.body(Some(&JsValue::from_str(str_json.as_str())));

        let url = content::exec_chat_url;
        let request = Request::new_with_str_and_init(url, &opts).unwrap();

        // 2、使用 webapi 发送请求
        let window = web_sys::window()
            .ok_or("no windows exists".to_string())
            .unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap();

        // 3、解析Response响应
        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();
        let resp = JsFuture::from(resp.json().unwrap()).await.unwrap();
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

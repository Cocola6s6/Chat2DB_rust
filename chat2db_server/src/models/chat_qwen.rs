use crate::models::db::Db;
use crate::models::prompt::PromptTemplate;
use anyhow::{Ok, Result};
use askama::Template;
use reqwest::{header::AUTHORIZATION, Client, Method, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

/**
 * {
    "model": "qwen-v1",
    "input": {
        "messages": [
            {
                "role": "system",
                "content": "你是达摩院的生活助手机器人。"
            },
            {
                "role": "user",
                "content": "你好，附近哪里有博物馆？"
            }
        ]
    }
}
 */

const BASE_URL: &str =
    "https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation";

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatQwen {
    pub model: String,
    pub input: Input,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub messages: Messages,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Messages {
    pub message: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatQwenResponse  {
    pub output: Output,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output  {
    pub text: String,
}


impl ChatQwen {
    pub fn new(&self, openai_key: String) -> ChatQwen {
        ChatQwen {
            model: "qwen-v1".to_string(),
            input: todo!(),
        }
    }

    // execute_chat
    pub async fn exec_chat(
        openai_key: &str,
        db_url: &str,
        db_ns: &str,
        text: &str,
    ) -> Result<String> {
        let db_url = db_url.to_string();
        let db_ns = db_ns.clone();
        let context = Db::query_schema(&db_url, &db_ns).await?;
        let promptTemp = PromptTemplate { context: &context };
        let prompt = promptTemp.render()?;
        // println!("prompt={:?}", prompt);

        let mut messages = vec![Message {
            role: "system".to_string(),
            content: prompt,
        }];

        messages.push(Message {
            role: "user".to_string(),
            content: text.to_string(),
        });

        let chat_qwen = ChatQwen {
            model: "qwen-v1".to_string(),
            input: Input { messages: Messages { message: messages } },
        };
        println!("chat_qwen={:?}", chat_qwen);
        println!("chat_qwen_json={:?}", json!(chat_qwen));

        let client = Client::new();
        let request = client.request(Method::POST, BASE_URL);

        let resp = request
            .header("Authorization", openai_key)
            .json(&json!(chat_qwen))
            .send()
            .await?;

        let resp: ChatQwenResponse = resp.json().await?;
        println!("{:?}", resp);

        Ok("".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_exec_chat() {
        let openai_key = "sk-34a5ce02952b436bb955dab064177c20".to_string();
        let db_url = "postgres://postgres:postgres@45.128.222.100:15432".to_string();
        let db_ns = "public".to_string();
        let text = "select * from public.test";
        let resp = ChatQwen::exec_chat(&openai_key, &db_url, &db_ns, &text)
            .await
            .unwrap();
        println!("resp={:?}", resp);
    }
}

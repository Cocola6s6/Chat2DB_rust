use crate::models::db::Db;
use crate::models::prompt::PromptTemplate;
use anyhow::{Ok, Result};
use askama::Template;
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use serde_json::json;

/**
 * 入参
{
    "model": "qwen-plus",
    "messages": [
        {
            "role": "system",
            "content": "You are a helpful assistant."
        },
        {
            "role": "user",
            "content": "你是谁？"
        }
    ]
}
 */

/**
 * 返回
{
    "choices": [
        {
            "message": {
                "role": "assistant",
                "content": "我是阿里云开发的一款超大规模语言模型，我叫通义千问。"
            },
            "finish_reason": "stop",
            "index": 0,
            "logprobs": null
        }
    ]
}
 */

const BASE_URL: &str =
    "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions";

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatQwen {
    pub model: String,
    pub messages: Vec<Message>,
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
pub struct ChatQwenResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
}

impl ChatQwen {
    pub fn new(&self, openai_key: String) -> ChatQwen {
        ChatQwen {
            model: "qwen-plus".to_string(),
            messages: todo!(),
        }
    }

    // execute_chat
    pub async fn exec_chat(
        openai_key: &str,
        db_url: &str,
        db_ns: &str,
        text: &str,
    ) -> Result<String> {
        // 1、准备prompt
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
            model: "qwen-plus".to_string(),
            messages: messages,
        };
        // println!("chat_qwen={:?}", chat_qwen);
        // println!("chat_qwen_json={}", json!(&chat_qwen));


        // 2、请求API
        let client = Client::new();
        let request = client.request(Method::POST, BASE_URL);

        let resp = request
            .header("Authorization", "Bearer ".to_string() + openai_key)
            .json(&json!(chat_qwen))
            .send()
            .await?;

        // 3、解析响应
        let resp: ChatQwenResponse = resp.json().await?;
        println!("resp={:?}", resp);

        let resp = resp.choices[0].message.content.clone();
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_exec_chat() {
        let openai_key = "sk-510ed600fa2342ffbb88d53931bb70b0".to_string();
        let db_url = "postgres://postgres:postgres@115.190.35.167:5432".to_string();
        let db_ns = "public".to_string();
        let text = "查询test_tb下的所有数据";
        let resp = ChatQwen::exec_chat(&openai_key, &db_url, &db_ns, &text)
            .await
            .unwrap();
        println!("resp={:?}", resp);
    }

    #[test]
    async fn test_resp() {
        let message: Message = serde_json::from_str(
            r#"{
            "role": "assistant",
            "content": "我是阿里云开发的一款超大规模语言模型，我叫通义千问。"
        }"#,
        )
            .unwrap();

        let resp = ChatQwenResponse {
            choices: vec![Choice {
                message: message,
                finish_reason: "stop".to_string(),
            }],
        };

        let jsonResp = serde_json::to_string(&resp).unwrap();

        println!("jsonResp={:?}", jsonResp);
    }
}

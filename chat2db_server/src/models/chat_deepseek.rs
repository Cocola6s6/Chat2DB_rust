use anyhow::{Ok, Result};
use askama::Template;
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::models::prompt::PromptTemplate;

/**
 * 入参
{
  "messages": [
    {
      "content": "You are a helpful assistant",
      "role": "system"
    },
    {
      "content": "Hi",
      "role": "user"
    }
  ],
  "model": "deepseek-chat",
  "frequency_penalty": 0,
  "max_tokens": 2048,
  "presence_penalty": 0,
  "response_format": {
    "type": "text"
  },
  "stop": null,
  "stream": false,
  "stream_options": null,
  "temperature": 1,
  "top_p": 1,
  "tools": null,
  "tool_choice": "none",
  "logprobs": false,
  "top_logprobs": null
}
 */

/**
 * 返回
{
  "id": "930c60df-bf64-41c9-a88e-3ec75f81e00e",
  "choices": [
    {
      "finish_reason": "stop",
      "index": 0,
      "message": {
        "content": "Hello! How can I help you today?",
        "role": "assistant"
      }
    }
  ],
  "created": 1705651092,
  "model": "deepseek-chat",
  "object": "chat.completion",
  "usage": {
    "completion_tokens": 10,
    "prompt_tokens": 16,
    "total_tokens": 26
  }
}
 */

const BASE_URL: &str =
    "https://api.deepseek.com/chat/completions";

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatDs {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatQwenResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    pub message: Message,
    pub finish_reason: String,
}

impl ChatDs {
    pub async fn exec_chat(text: &str, openai_key: &str) -> Result<String> {
        let context = text;
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


        let chat_ds = ChatDs {
            model: "deepseek-chat".to_string(),
            messages: messages,
        };
        // println!("chat_ds={:?}", chat_qwen);
        // println!("chat_ds_json={}", json!(&chat_qwen));


        // 2、请求API
        let client = Client::new();
        let request = client.request(Method::POST, BASE_URL);

        let resp = request
            .header("Authorization", "Bearer ".to_string() + openai_key)
            .json(&json!(chat_ds))
            .send()
            .await?;

        // 3、解析响应
        let resp: ChatQwenResponse = resp.json().await?;
        // println!("resp={:?}", resp);

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
        let text = "The picture does not fit";
        let openai_key = "sk-da4796912b01421d9be824b50473ab98";
        let resp = ChatDs::exec_chat(&text, &openai_key)
            .await
            .unwrap();
        println!("resp={:?}", resp);
    }
}

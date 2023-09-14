use crate::models::message::Message;
use crate::models::prompt::PromptTemplate;
use crate::models::db::Db;
use anyhow::{Ok, Result};
use askama::Template;
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};
use serde::{Deserialize, Serialize};

// TODO Chat结构体待优化，因为和Sql结构体关联了作为VO
#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub openai_key: String,
    pub db: Db,
    pub text: String,
}

impl Chat {
    // get a new instance of Chat
    pub fn new(&self, openai_key: String) -> Chat {
        Chat {
            openai_key,
            db: todo!(),
            text: todo!(),
        }
    }

    // execute_chat
    pub async fn exec_chat(openai_key: &str, db_url: &str, db_ns: &str, text: &str) -> Result<String> {
        set_key(openai_key.to_string());

        let db_url = db_url.to_string();
        let db_ns = db_ns.clone();
        let context = Db::query_schema(&db_url, &db_ns).await?;
        let promptTemp = PromptTemplate { context: &context };
        let prompt = promptTemp.render()?;
        // println!("{}", prompt);

        let mut messages = vec![ChatCompletionMessage {
            role: ChatCompletionMessageRole::System,
            content: Some(prompt),
            name: None,
            function_call: None,
        }];

        messages.push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some(text.to_string()),
            name: None,
            function_call: None,
        });

        let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages.clone())
            .create()
            .await?;
        let returned_message: ChatCompletionMessage =
            chat_completion.choices.first().unwrap().message.clone();

        println!(
            "{:#?}: {}",
            &returned_message.role,
            &returned_message.content.clone().unwrap().trim()
        );

        messages.push(returned_message);

        let json_str = messages.last().unwrap().content.clone().unwrap();
        let message: Message = serde_json::from_str(&json_str)?;

        Ok(message.result)
    }
}

// override default values with environment variables
impl Default for Chat {
    fn default() -> Self {
        Chat {
            openai_key: std::env::var("OPENAI_KEY").unwrap_or_else(|_| "".to_string()),
            db: todo!(),
            text: todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use dotenv::dotenv;

    #[test]
    async fn test_exec_chat() {
        dotenv().unwrap();
        let openai_key = std::env::var("OPENAI_KEY").unwrap_or_else(|_| "".to_string());
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "".to_string());
        let db_ns = std::env::var("DATABASE_NS").unwrap_or_else(|_| "".to_string());
        let text = "Hello?";
        let resp = Chat::exec_chat(&openai_key, &db_url, &db_ns, &text).await.unwrap();
        println!("resp={:#?}", resp);
    }
}

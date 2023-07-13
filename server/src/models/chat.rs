use crate::models::message::Message;
use crate::models::prompt::PromptTemplate;
use crate::models::sql::Sql;
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
    pub sql: Sql,
    pub text: String,
}

impl Chat {
    // get a new instance of Chat
    pub fn new(&self, openai_key: String) -> Chat {
        Chat {
            openai_key,
            sql: todo!(),
            text: todo!(),
        }
    }

    // execute_chat
    pub async fn execute_chat(openai_key: &str, url: &str, ns: &str, text: &str) -> Result<String> {
        println!("[do_chat]=======================>");
        set_key(openai_key.to_string());

        let db_url = url.to_string();
        let db_ns = ns.clone();
        let context = Sql::query_schema(&db_url, &db_ns).await?;
        let promptTemp = PromptTemplate { context: &context };
        let prompt = promptTemp.render()?;
        // println!("{}", prompt);

        let mut messages = vec![ChatCompletionMessage {
            role: ChatCompletionMessageRole::System,
            content: Some(prompt),
            name: None,
            function_call: None,
        }];

        // print!("User: ");
        // stdout().flush()?;

        let mut user_message_content = String::new();

        // stdin().read_line(&mut user_message_content)?;
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

        // TODO 执行sql操作后续放到client端执行
        // Sql::execute_sql(&db_url, &message.result, message.code).await?;

        Ok(message.result)
    }
}

// override default values with environment variables
impl Default for Chat {
    fn default() -> Self {
        Chat {
            openai_key: std::env::var("OPENAI_KEY").unwrap_or_else(|_| "".to_string()),
            sql: todo!(),
            text: todo!(),
        }
    }
}

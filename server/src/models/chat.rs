use crate::models::message::Message;
use crate::models::prompt::PromptTemplate;
use crate::models::sql::Sql;
use anyhow::{Result, Ok};
use askama::Template;
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};
use serde::{Deserialize, Serialize};
use std::io::{stdin, stdout, Write};

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

    // do_chat
    pub async fn do_chat(&self, sql: &Sql) -> Result<()> {
        println!("[do_chat]=======================>");
        set_key(self.openai_key.clone());

        let db_url = sql.url.clone();
        let db_ns = sql.ns.clone();
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
            content: Some(self.text.clone()),
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
        Sql::execute_sql(&db_url, &message.result, message.code).await?;

        Ok(())
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

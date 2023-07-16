use anyhow::Result;
use chat2db_server::{common::read_utils, models::chat::Chat};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let openai_key = read_utils::get_openai_key();
    let db_url = read_utils::get_db_url();
    let db_ns = read_utils::get_db_ns();
    let text = read_utils::read_input("Enter your text:").unwrap_or_default();

    let resp = Chat::exec_chat(&openai_key, &db_url, &db_ns, &text)
        .await
        .unwrap();
    println!("resp={:#?}", resp);
}

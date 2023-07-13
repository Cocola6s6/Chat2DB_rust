use server::models::chat::Chat;
use server::models::sql::Sql;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let sql: Sql = Sql::default();
    let chat = Chat::default();
    let openai_key = chat.openai_key.clone();
    let url = sql.url.clone();
    let ns = sql.ns.clone();
    let text = chat.text.clone();
    Chat::execute_chat(&openai_key, &url, &ns, &text).await.unwrap();
}

use server::models::chat::Chat;
use server::models::db::Db;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let sql: Db = Db::default();
    let chat = Chat::default();
    let openai_key = chat.openai_key.clone();
    let url = sql.db_url.clone();
    let ns = sql.db_ns.clone();
    let text = chat.text.clone();
    Chat::exec_chat(&openai_key, &url, &ns, &text).await.unwrap();
}

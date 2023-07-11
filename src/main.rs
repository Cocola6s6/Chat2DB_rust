use Chat2DB_rust::models::chat::Chat;
use Chat2DB_rust::models::sql::Sql;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let sql: Sql = Sql::default();
    let chat = Chat::default();
    chat.do_chat(&sql).await.unwrap();
}

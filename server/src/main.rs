use server::models::chat::Chat;
use server::models::sql::Sql;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let sql: Sql = Sql::default();
    let chat = Chat::default();
    chat.do_chat(&sql).await.unwrap();
}

use anyhow::Result;
use chat2db_server::{common::read_utils, models::db::Db};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let db_url = read_utils::get_db_url();

    let cmd_options = [("1", "exec_sql"), ("2", "query_tables")];
    loop {
        println!("Please choose one of the following options:");
        for (option, desc) in cmd_options {
            println!("{}:{}", option, desc);
        }

        let cmd = read_utils::read_input("Your choice is:").unwrap_or_default();
        let cmd = if cmd.is_empty() { "1".to_string() } else { cmd };

        if cmd.eq("exit") {
            break;
        }

        match cmd.as_str() {
            "1" => {
                let sql = read_utils::get_sql();
                let resp = Db::exec_sql(&db_url, &sql, 2).await.unwrap();
                println!("resp={:#?}", resp);
            }
            "2" => {
                let db_ns = read_utils::get_db_ns();

                let resp = Db::query_tables(&db_url, &db_ns).await.unwrap();
                println!("resp={:#?}", resp);
            }
            _ => println!("Invalid choice: {}", cmd),
        }
    }
}

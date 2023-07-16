use anyhow::Result;
use std::io::{self, BufRead};

pub fn read_input(cmd: &str) -> Result<String> {
    println!("{}", cmd);
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn get_openai_key() -> String {
    let openai_key = read_input("Enter OPENAI_KEY:").unwrap_or_default();

    // 如果不输入从配置文件中读取
    let env_key = std::env::var("OPENAI_KEY").unwrap();
    let openai_key = if openai_key.is_empty() { env_key } else { openai_key };

    openai_key
}

pub fn get_db_url() -> String {
    let db_url = read_input("Enter DB_URL:").unwrap_or_default();

    // 如果不输入从配置文件中读取
    let env_url = std::env::var("DATABASE_URL").unwrap();
    let db_url = if db_url.is_empty() { env_url } else { db_url };

    db_url
}

pub fn get_db_ns() -> String {
    let db_ns = read_input("Enter DB_NS:").unwrap_or_default();

    // 如果不输入从配置文件中读取
    let env_ns = std::env::var("DATABASE_NS").unwrap();
    let db_ns = if db_ns.is_empty() { env_ns } else { db_ns };

    db_ns
}

pub fn get_sql() -> String {
    let sql = read_input("Enter sql:").unwrap_or_default();

    sql
}  
use anyhow::{Ok, Result};
use db_schema::PgSchema;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct Db {
    pub db_url: String,
    pub db_ns: String,
}

impl Db {
    // query_schema
    pub async fn query_schema(db_url: &str, db_ns: &str) -> Result<String> {
        let pool = PgPool::connect(db_url).await?;
        let schema = PgSchema::new(db_ns);

        let enums = schema.get_enums(&pool).await?.join("\n");
        let types = schema.get_types(&pool).await?.join("\n");
        let tables = schema.get_tables(&pool).await?.join("\n");
        let views = schema.get_views(&pool).await?.join("\n");
        let mviews = schema.get_mviews(&pool).await?.join("\n");
        let functions = schema.get_functions(&pool).await?.join("\n");
        let triggers = schema.get_triggers(&pool).await?.join("\n");
        let indexes = schema.get_indexes(&pool).await?.join("\n");

        let mut all = String::new();
        all.push_str(&enums);
        all.push_str(&types);
        all.push_str(&tables);
        all.push_str(&views);
        all.push_str(&mviews);
        all.push_str(&functions);
        all.push_str(&triggers);
        all.push_str(&indexes);

        Ok(all)
    }

    // query_enums
    pub async fn query_tables(db_url: &str, db_ns: &str) -> Result<Vec<String>> {
        let pool = PgPool::connect(db_url).await?;
        let schema = PgSchema::new(db_ns);
        let tables = schema.get_tables(&pool).await?;

        let vec = tables
            .into_iter()
            .map(|table| {
                let re = Regex::new(r"CREATE TABLE ([^\s\(]+)").unwrap();
                if let Some(captures) = re.captures(&table) {
                    if let Some(table_name) = captures.get(1) {
                        return table_name.as_str().to_string();
                    }
                }
                return "".to_string();
            })
            .collect::<Vec<String>>();

        Ok(vec)
    }

    // execute_sql
    pub async fn exec_sql(db_url: &str, sql: &str, code: u32) -> Result<()> {
        match code {
            2 => {
                let pool = PgPool::connect(db_url).await?;
                let result = sqlx::query(sql).execute(&pool).await?;
                println!("{:#?}", result);
            }
            _ => {
                println!("");
            }
        }

        Ok(())
    }
}

// override default values with environment variables
impl Default for Db {
    fn default() -> Db {
        Db {
            db_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| "".to_string()),
            db_ns: std::env::var("DATABASE_NS").unwrap_or_else(|_| "".to_string()),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use dotenv::dotenv;

    #[test]
    async fn test_query_schema() {
        dotenv().unwrap();
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "".to_string());
        let db_ns = std::env::var("DATABASE_NS").unwrap_or_else(|_| "".to_string());
        let resp = Db::query_schema(&db_url, &db_ns).await.unwrap();
        println!("resp={:#?}", resp);
    }

    #[test]
    async fn test_query_tables() {
        dotenv().unwrap();
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "".to_string());
        let db_ns = std::env::var("DATABASE_NS").unwrap_or_else(|_| "".to_string());
        let resp = Db::query_tables(&db_url, &db_ns).await.unwrap();
        println!("resp={:#?}", resp);
    }

    #[test]
    async fn test_exec_sql() {
        dotenv().unwrap();
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "".to_string());
        let sql = "select * from test";
        let code = 2;
        let resp = Db::exec_sql(&db_url, &sql, code).await.unwrap();
        println!("resp={:#?}", resp);
    }
}


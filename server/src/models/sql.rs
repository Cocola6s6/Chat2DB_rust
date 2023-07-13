use anyhow::{Ok, Result};
use db_schema::PgSchema;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sql {
    pub url: String,
    pub ns: String,
}

impl Sql {
    // query_schema
    pub async fn query_schema(url: &str, ns: &str) -> Result<String> {
        let pool = PgPool::connect(url).await?;
        let schema = PgSchema::new(ns);

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
    pub async fn query_tables(url: &str, ns: &str) -> Result<Vec<String>> {
        let pool = PgPool::connect(url).await?;
        let schema = PgSchema::new(ns);
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
    pub async fn execute_sql(url: &str, sql: &str, code: u32) -> Result<()> {
        match code {
            2 => {
                let pool = PgPool::connect(url).await?;
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
impl Default for Sql {
    fn default() -> Sql {
        Sql {
            url: std::env::var("DATABASE_URL").unwrap_or_else(|_| "".to_string()),
            ns: std::env::var("DATABASE_NS").unwrap_or_else(|_| "".to_string()),
        }
    }
}

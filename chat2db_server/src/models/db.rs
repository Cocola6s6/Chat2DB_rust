use std::collections::{HashMap, BTreeMap};

use anyhow::{Ok, Result};
use db_schema::PgSchema;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::{Column, PgPool, Row, TypeInfo};

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

    // query_tables
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
    pub async fn exec_sql(
        db_url: &str,
        sql: &str,
        code: u32,
    ) -> Result<Vec<BTreeMap<String, String>>> {
        // 声明list保存所有行数据
        let mut list: Vec<BTreeMap<String, String>> = Vec::new();

        match code {
            2 => {
                // 得到查询结果，所有的字段和值
                let pool = PgPool::connect(db_url).await?;
                let result = sqlx::query(sql).fetch_all(&pool).await?;

                // 遍历查询结果的行
                for row in result {
                    let columns = row.columns();

                    // 声明map保存行数据
                    let mut map: BTreeMap<String, String> = BTreeMap::new();

                    // 根据ordinal排序，固定字段顺序
                    let mut sorted_columns = columns.iter().collect::<Vec<_>>();
                    sorted_columns.sort_by_key(|c| c.ordinal());
                    // println!("sorted_columns={:?}", sorted_columns);

                    for (i, column) in sorted_columns.iter().enumerate() {
                        // 获取字段名
                        let name = column.name();
                        let type_info = column.type_info();
                        // let ordinal = column.ordinal();
                        // println!("name={:?}, type_info={:?}, ordinal={:?}", name, type_info, ordinal);

                        // 获取字段值
                        // TODO 统一返回类型，使用serde_json::Value，否则会类型不匹配报错
                        let value: serde_json::Value = match type_info.name() {
                            "INT4" => serde_json::Value::Number(row.get::<i32, _>(i).into()),
                            "VARCHAR" => serde_json::Value::String(row.get::<String, _>(i).into()),
                            _ => serde_json::Value::Null,
                        };

                        // let value = row.get(i);

                        let value = value.to_string().replace("\"", "");
                        println!("name={:?}, value={:?}", name, value);

                        map.insert(name.to_string(), value);
                        // println!("map={:?}", map)
                    }

                    list.push(map);
                }

                println!("list={:?}", list);
            }

            _ => {
                println!("");
            }
        }

        return Ok(list);
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
    use dotenv::dotenv;
    use tokio::test;

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
        let sql = "select * from test2";
        let code = 2;
        let resp = Db::exec_sql(&db_url, &sql, code).await.unwrap();
        println!("resp={:#?}", resp);
    }
}

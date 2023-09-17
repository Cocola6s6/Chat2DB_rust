use anyhow::{Ok, Result};
use moka::future::Cache;
use once_cell::sync::Lazy;

use crate::models::connection::Connection;

// 本地缓存用户连接
static CACHE: Lazy<Cache<usize, Connection>> = Lazy::new(|| Cache::new(10_000));

// static CACHE: Cache<usize, Connection> = Cache::new(10_000);

// static mut CACHE: LruCache<usize, Connection> = LruCache::new(NonZeroUsize::new(2).unwrap());

pub struct CacheUtils();

impl CacheUtils {
    pub async fn add_cache(key: usize, value: Connection) -> Result<()> {
        let _ = CACHE.insert(key, value).await;
        // println!("Value for key1: {:?}", CACHE.get(&1));

        Ok(())
    }

    pub async fn rm_cache(console_id: usize) -> Result<()> {
        let _ = CACHE.remove(&console_id).await;

        Ok(())
    }

    pub async fn get_cache(console_id: usize) -> Result<Connection> {
        let cache = CACHE.get(&console_id).unwrap();

        Ok(cache)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::db::Db;

    use super::*;
    use dotenv::dotenv;
    use tokio::test;

    #[test]
    async fn test_conn() {
        dotenv().unwrap();
        let openai_key = std::env::var("OPENAI_KEY").unwrap_or_else(|_| "".to_string());
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "".to_string());
        let db_ns = std::env::var("DATABASE_NS").unwrap_or_else(|_| "".to_string());
        let conn = Connection {
            console_id: 1,
            openai_key,
            db: Db { db_url, db_ns },
        };
        let resp = CacheUtils::add_cache(1, conn).await.unwrap();
        println!("resp={:#?}", resp);
    }

    #[test]
    async fn test_dis_conn() {
        let console_id = 1;
        let resp = CacheUtils::rm_cache(console_id).await.unwrap();
        println!("resp={:#?}", resp);
    }

    #[test]
    async fn get_conn() {
        let console_id = 1;
        let resp = CacheUtils::get_cache(console_id).await.unwrap();
        println!("resp={:#?}", resp);
    }
}

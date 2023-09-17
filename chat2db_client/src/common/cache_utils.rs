use tracing::info;
use web_sys::{window, Storage};

pub struct CacheUtils {}

impl CacheUtils {
    // 获取本地存储
    pub fn local_storage() -> Option<Storage> {
        window().and_then(|win| win.local_storage().ok()).flatten()
    }

    // 获取本地缓存中的数据
    pub fn get_local_data(key: &str) -> Option<String> {
        Self::local_storage()
            .and_then(|storage| storage.get_item(key).ok())
            .flatten()
    }

    // 设置数据到本地缓存
    pub fn set_local_data(key: &str, value: &str) {
        if let Some(storage) = Self::local_storage() {
            if let Err(err) = storage.set_item(key, value) {
                info!("{}", format!("Failed to set local storage: {:?}", err));
            }
        }
    }
}

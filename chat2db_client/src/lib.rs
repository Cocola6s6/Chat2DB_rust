pub mod components;
pub mod models;
pub mod common;

use models::chat::Chat;
use models::db::Db;
use sycamore::reactive::{RcSignal, create_rc_signal};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AppState {
    pub chat: RcSignal<Chat>,
    pub db: RcSignal<Db>,
}

impl AppState {
    pub async fn new() -> Self {
        // TODO 读取不到.env配置文件
        let chat = create_rc_signal(Chat::default());
        let db = create_rc_signal(Db::default());

        Self {
            chat,
            db,
        }
    }
 }

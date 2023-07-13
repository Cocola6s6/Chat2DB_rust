pub mod components;
pub mod models;
use models::chat::Chat;
use models::sql::Sql;
use sycamore::reactive::{RcSignal, create_rc_signal};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AppState {
    pub chat: RcSignal<Chat>,
    pub sql: RcSignal<Sql>,
}

impl AppState {
    pub async fn new() -> Self {
        // TODO 读取不到.env配置文件
        let chat = create_rc_signal(Chat::default());
        let sql = create_rc_signal(Sql::default());

        Self {
            chat,
            sql,
        }
    }
 }

use crate::AppState;
use crate::{components::chatinput::Chatinput, components::connection::Connection};
use sycamore::prelude::*;
use tracing::info;

// App 组件
#[component]
pub async fn App<G: Html>(ctx: Scope<'_>) -> View<G> {
    init(ctx).await;

    view! {ctx,
        // TODO 删除：连接组件, 背景颜色为红色
        div (class="bg-red-200") {
            Connection()
        }
        // TODO 删除：聊天组件, 背景颜色为绿色
        div (class="bg-green-200") {
            Chatinput()
        }
    }
}

pub async fn init(ctx: Scope<'_>) {
    let state = AppState::new().await;
    info!("AppState init done]===================>");
    info!("{:?}", state);

    provide_context(ctx, state);
    info!("ctx context init done]===================>");
}

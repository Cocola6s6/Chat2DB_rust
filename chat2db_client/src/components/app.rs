use crate::AppState;
use crate::{components::chatinput::Chatinput, components::connection::Connection};
use sycamore::prelude::*;
use tracing::info;

// App 组件
#[component]
pub async fn App<G: Html>(ctx: Scope<'_>) -> View<G> {
    init(ctx).await;

    view! {ctx,
        div {
            Connection()
        }
        div {
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

use crate::AppState;
use crate::{components::chatinput::Chatinput, components::connection::Connection};
use crate::routers::router::switch;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Router};
use tracing::info;

// App 组件
#[component]
pub async fn App<G: Html>(ctx: Scope<'_>) -> View<G> {
    init(ctx).await;

    view! {ctx,
        div(class="text-sm font-medium text-center text-gray-500 border-b border-gray-200 dark:text-gray-400 dark:border-gray-700"){
            ul(class="flex flex-wrap -mb-px") {
                li(class="mr-2") {
                    a(href="/connection", class="inline-block p-4 border-b-2 border-transparent rounded-t-lg hover:text-gray-600 hover:border-gray-300 dark:hover:text-gray-300") {
                        ("Connection")
                    }
                }
                li(class="mr-2") {
                    a(href="/chatinput", class="inline-block p-4 border-b-2 border-transparent rounded-t-lg hover:text-gray-600 hover:border-gray-300 dark:hover:text-gray-300") {
                        ("Input")
                    }
                }
            }
        }

        
        // TODO 使用路由，替换掉下面的静态。存在问题：1）页面刷新了，当前上下文中的AppState传递不到下一个页面
        Router(
            integration=HistoryIntegration::new(),
            view=switch,

        )


        // // TODO 删除：连接组件, 背景颜色为红色
        // div (class="bg-red-200") {
        //     Connection()
        // }
        // // TODO 删除：聊天组件, 背景颜色为绿色
        // div (class="bg-green-200") {
        //     Chatinput()
        // }

    }
}

pub async fn init(ctx: Scope<'_>) {
    let state = AppState::new().await;
    info!("AppState init done]===================>");
    info!("{:?}", state);

    provide_context(ctx, state);
    info!("ctx context init done]===================>");
}

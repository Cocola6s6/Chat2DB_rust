use crate::models::chat::Chat;
use crate::models::connection::Connection;
use crate::models::db::Db;
use crate::AppState;
use rand::Rng;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use tracing::info;

// connection 组件
#[component]
pub fn Connection<G: Html>(ctx: Scope<'_>) -> View<G> {
    let openai_key_signal = create_signal(ctx, String::from(""));
    let db_url_signal = create_signal(ctx, String::from(""));
    let db_ns_signal = create_signal(ctx, String::from(""));

    let connection_btn_event = move |_| {
        info!("[button_event_listener_2]=======================>");
        spawn_local_scoped(ctx, async move {
            // 本地缓存
            set_state(ctx, openai_key_signal.get().to_string(), db_url_signal.get().to_string(), db_ns_signal.get().to_string()).await;

            // 服务器本地缓存
            let _ = Connection::conn(openai_key_signal.get().to_string(), db_url_signal.get().to_string(), db_ns_signal.get().to_string())
                .await
                .unwrap_or_default();
        })
    };

    view! {ctx,
        // TODO 输入需要反显回输入信息。
        form (class="gap-6 mb-6 md:grid-cols-2") {
            div() {
                div() {
                    label(class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                        "OPENAI_KEY:"
                    }
                    input(
                        bind:value=openai_key_signal,
                        type="text", id="openai_key",
                        class="block mx-4 p-2.5 w-1/2 text-sm text-gray-900 bg-white rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-800 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    ) {
                    }
                }

                div() {
                    label(class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                        "DB_URL:"
                    }
                    input(
                        bind:value=db_url_signal,
                        type="text",
                        id="db_url",
                        class="block mx-4 p-2.5 w-1/2 text-sm text-gray-900 bg-white rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-800 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    ) {
                    }
                }

                div() {
                    label(class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                        "DB_NS:"
                    }
                    input(
                        bind:value=db_ns_signal,
                        type="text",
                         id="db_ns",
                        class="block mx-4 p-2.5 w-1/2 text-sm text-gray-900 bg-white rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-800 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    ) {
                    }
                }
            }
        }

        div(class="grid gap-6 mb-6 md:grid-cols-2") {
            button(
                on:click=connection_btn_event,
                id="connection_btn",
                class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
            ) {
                "Connection"
            }
        }
    }
}

// 设置上下文Appstate
pub async fn set_state(ctx: Scope<'_>, openai_key: String, db_url: String, db_ns: String) {
    let chat = Chat { openai_key };
    let sql = Db { db_url, db_ns };

    let state = use_context::<AppState>(ctx);
    info!(
        "[set_state]==============================>{:?}, {:?}",
        chat, sql
    );
    state.chat.set(chat);
    state.db.set(sql);
}

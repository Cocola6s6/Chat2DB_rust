use crate::components::chatoutput::Chatoutput;
use crate::models::chat::Chat;
use crate::models::db::Db;
use crate::AppState;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use tracing::info;

// input 组件
#[component]
pub async fn Chatinput<G: Html>(ctx: Scope<'_>) -> View<G> {
    let state = use_context::<AppState>(ctx);

    let text_signal = create_signal(ctx, String::from(""));
    let chat_ouput_signal = create_signal(ctx, String::from(""));
    let db_ouput_signal = create_signal(ctx, Vec::new());
    let db_ouput_keys_signal = create_signal(ctx, Vec::new());
    let query_tables_ouput_signal = create_signal(ctx, Vec::new());

    let ask_btn_event = move |_| {
        spawn_local_scoped(ctx, async move {
            // 1、获取输入框内容
            let text = text_signal.get().to_string();

            // 2、请求chatapi
            let resp = Chat::exec_chat(
                state.chat.get().openai_key.clone(),
                state.db.get().db_url.clone(),
                state.db.get().db_ns.clone(),
                text,
            )
            .await
            .unwrap_or_default();

            chat_ouput_signal.set(resp);
        })
    };

    let exec_btn_event = move |_| {
        spawn_local_scoped(ctx, async move {
            // 1、获取要执行的sql
            let sql = if !chat_ouput_signal.get().is_empty() {
                chat_ouput_signal.get().to_string()
            } else {
                text_signal.get().to_string()
            };

            // 2、请求dbapi
            let resp = Db::exec_sql(state.db.get().db_url.clone(), sql)
                .await
                .unwrap_or_default();

            let keys: Vec<String> = resp
                .get(0)
                .iter()
                .flat_map(|dict| dict.keys().cloned())
                .collect();
            info!("Keys: {:?}", keys);
            db_ouput_keys_signal.set(keys);

            db_ouput_signal.set(resp);
        })
    };

    let query_tables_btn_event = move |_| {
        spawn_local_scoped(ctx, async move {
            // 请求dbapi
            let resp =
                Db::query_tables(state.db.get().db_url.clone(), state.db.get().db_ns.clone())
                    .await
                    .unwrap_or_default();

            query_tables_ouput_signal.set(resp);
        })
    };

    view! {ctx,
            // TODO 删除：表单组件，背景颜色为黄色
            div (class="bg-yellow-200") {
                form() {
                    label(class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                        "YOUR MESSAGE:"
                    }
                    div(class="grid gap-6 mb-6 md:grid-cols-2") {
                        textarea(
                            bind:value=text_signal,
                            id="text",
                            rows="1",
                            style="height: 256px;",
                            class="block mx-4 p-2.5 w-full text-sm text-gray-900 bg-white rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-800 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                            placeholder="Your message...") {
                        }
                    }
                }
                div(class="grid gap-6 mb-6 md:grid-cols-2") {
                    button(
                        on:click=ask_btn_event,
                        id="ask_btn",
                        class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800") {
                        "Ask"
                    }
                }
                div(class="grid gap-6 mb-6 md:grid-cols-2") {
                    button(
                        on:click=exec_btn_event,
                        id="exec_btn",
                        class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800") {
                        "Exec"
                    }
                }
                div(class="grid gap-6 mb-6 md:grid-cols-2") {
                    button(
                        on:click=query_tables_btn_event,
                        id="query_tables_btn",
                        class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800") {
                        "Tables"
                    }
                }
            }

            // TODO 删除：显示组件，背景颜色为紫色
            div (class="bg-purple-200") {
                Chatoutput(
                    chat_output_text=chat_ouput_signal,
                    db_output_text=db_ouput_signal,
                    db_ouput_keys_text=db_ouput_keys_signal,
                    tables_output_text=query_tables_ouput_signal
                )
            }
    }
}

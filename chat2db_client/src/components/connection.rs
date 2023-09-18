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
            set_state(
                ctx,
                openai_key_signal.get().to_string(),
                db_url_signal.get().to_string(),
                db_ns_signal.get().to_string(),
            )
            .await;

            // 服务器本地缓存
            let _ = Connection::conn(
                openai_key_signal.get().to_string(),
                db_url_signal.get().to_string(),
                db_ns_signal.get().to_string(),
            )
            .await
            .unwrap_or_default();
        })
    };

    view! {ctx,
        // TODO 输入需要反显回输入信息。
        div(class="Frame w-[1200px] h-[800px] relative bg-white") {
            div(class="WelcomeToChat2dbRust w-[1083px] h-[46px] left-[24px] top-[222px] absolute text-center text-black text-4xl font-normal font-['Open Sans'] leading-9") {
                "Welcome to Chat2DB_Rust!"
            }

            div(class="WelcomeBackItSGreatToHaveYou w-[308px] left-[433px] top-[306px] absolute text-center text-slate-400 text-lg font-normal font-['Open Sans'] leading-[18px]") {
                "Welcome back it's great to have you!"
            }

            div(class=" w-[22px] left-[830px] top-[217px] absolute text-center text-black text-4xl font-normal font-['Open Sans'] leading-9") {
                "👋"
            }

            form {
                div {
                    label(class="Openaikey w-[87px] h-5 left-[351px] top-[380px] absolute text-black text-sm font-normal font-['Open Sans'] leading-[14px]") {
                        "OPENAI_KEY:"
                    }
                    input(
                        bind:value=openai_key_signal,
                        type="text", id="openai_key",
                        class="Openaikey w-[472px] h-[45px] left-[351px] top-[405px] absolute bg-white rounded border border-slate-300"
                    ) {}
                }

                div {
                    label(class="Dburl w-[87px] h-5 left-[351px] top-[466px] absolute text-black text-sm font-normal font-['Open Sans'] leading-[14px]") {
                        "DB_URL:"
                    }
                    input(
                        bind:value=db_url_signal,
                        type="text",
                        id="db_url",
                        class="Dburl w-[472px] h-[45px] left-[351px] top-[488px] absolute bg-white rounded border border-slate-300"
                    ) {}
                }

                div{
                    label(class="Dbns w-[87px] h-5 left-[351px] top-[547px] absolute text-black text-sm font-normal font-['Open Sans'] leading-[14px]") {
                        "DB_NS:"
                    }
                    input(
                        bind:value=db_ns_signal,
                        type="text",
                         id="db_ns",
                         class="Dbns w-[472px] h-[45px] left-[351px] top-[571px] absolute bg-white rounded border border-slate-300"
                    ) {}
                }
            }

            div {
                div(class="Connection w-[123px] h-[29px] left-[525px] top-[676px] absolute text-center text-white text-sm font-normal font-['Roboto']") {}
                button(
                    on:click=connection_btn_event,
                    id="connection_btn",
                    class="Connection w-[157px] h-[42px] left-[508px] top-[663px] absolute bg-blue-500 rounded"
                )
                {"Connection"}
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

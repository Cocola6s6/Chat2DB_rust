use crate::components::chatoutput::Chatoutput;
use crate::models::chat::Chat;
use crate::models::connection::Connection;
use crate::models::db::Db;
use crate::AppState;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use tracing::info;

// input 组件
#[component]
pub async fn Chatinput<'a, G: Html>(ctx: Scope<'a>) -> View<G> {
    get_conn(ctx).await;

    let state = use_context::<AppState>(ctx);
    info!("state={:?}=======================>", state);

    let text_signal = create_signal(ctx, String::from(""));
    let chat_ouput_signal = create_signal(ctx, String::from(""));
    let db_ouput_signal = create_signal(ctx, Vec::new());
    let query_tables_ouput_signal = create_signal(ctx, Vec::new());

    // 执行chat
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

    // 执行sql
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

            db_ouput_signal.set(resp);
        })
    };

    // 查询tables
    let resp = Db::query_tables(state.db.get().db_url.clone(), state.db.get().db_ns.clone())
        .await
        .unwrap_or_default();
    query_tables_ouput_signal.set(resp);


    view! {ctx,
        div(class="Input w-[1440px] h-[900px] relative bg-white") {

            // Title
            div(class="Title w-[1083px] h-[51px] left-[178px] top-[69px] absolute") {
                div(class=" w-[22px] left-[806px] top-0 absolute text-center text-black text-4xl font-normal font-['Open Sans'] leading-9") {
                    "👋"
                }
                div(class="WelcomeToChat2dbRust w-[1083px] h-[46px] left-0 top-[5px] absolute text-center text-black text-4xl font-normal font-['Open Sans'] leading-9") {
                    "Welcome to Chat2DB_Rust!"
                }
            }

            // Chat
            div(class="Ask w-[968px] h-[138px] left-[333px] top-[186px] absolute") {
                div(class="Asktips w-[246px] h-9 left-0 top-0 absolute") {
                    div(class="AskAiHere w-[197px] h-[15px] left-0 top-[18px] absolute text-black text-2xl font-normal font-['Open Sans'] leading-[14px]") {
                        "Ask AI Here"
                    }
                    div(class=" w-[22px] left-[224px] top-0 absolute text-center text-black text-4xl font-normal font-['Open Sans'] leading-9") {
                        "👋"
                    }
                    form {
                        textarea(
                            bind:value=text_signal,
                            id="text",
                            rows="1",
                            style="height: 150px;",
                            class="Openaikey w-[660px] h-[138px] left-[308px] top-0 absolute bg-white rounded border border-slate-300",
                            placeholder="Your message...") {
                        }
                    }
                }
                div(class="Askbutton w-[157px] h-[42px] left-[66px] top-[69px] absolute") {
                    div(class="Ask w-[157px] h-[42px] left-0 top-0 absolute bg-blue-500 rounded") {}
                    button(
                        on:click=ask_btn_event,
                        id="ask_btn",
                        class="Ask w-[123px] h-[29px] left-[17px] top-[13px] absolute text-center text-white text-sm font-normal font-['Roboto']") {
                        "Ask"
                    }
                }
            }

            // Exec
            div(class="Exec w-[968px] h-[105px] left-[333px] top-[371px] absolute") {
                div(class="Execbutton w-[157px] h-[42px] left-[66px] top-[63px] absolute") {
                    div(class="Exec w-[157px] h-[42px] left-0 top-0 absolute bg-blue-500 rounded") {}
                    button(
                        on:click=exec_btn_event,
                        id="exec_btn",
                        class="Exec w-[123px] h-[29px] left-[17px] top-[13px] absolute text-center text-white text-sm font-normal font-['Roboto']") {
                        "Exec"
                    }
                }
                div {
                    form {
                        textarea(
                            bind:value=chat_ouput_signal,
                            id="text",
                            rows="1",
                            // style="height: 30px;",
                            class="Sql w-[660px] h-14 left-[308px] top-[48px] absolute bg-white rounded border border-slate-300",
                            placeholder="Your message...") {
                        }
                    }
                }
                div(class="Exectips w-[246px] h-[30px] left-0 top-0 absolute") {
                    div(class="ExecSqlHere w-[193.85px] h-[12.50px] left-0 top-[15px] absolute text-black text-2xl font-normal font-['Open Sans'] leading-[14px]") {
                        "Exec SQL Here"
                    }
                    div(class=" w-[24.94px] h-[30px] left-[221.06px] top-0 absolute text-center text-black text-4xl font-normal font-['Open Sans'] leading-9") {
                        "👋"
                    }
                }
            }

            // output
            div {
                Chatoutput(
                    db_output_text=db_ouput_signal,
                    tables_output_text=query_tables_ouput_signal
                )
            }
        }  
    }
}


// 获取服务器本地缓存
async fn get_conn(ctx: Scope<'_>) {
    let connection = Connection::get_conn(1).await.unwrap();
    let chat = Chat {
        openai_key: connection.openai_key.clone(),
    };
    let db = Db {
        db_url: connection.db.db_url.clone(),
        db_ns: connection.db.db_ns.clone(),
    };

    let state = use_context::<AppState>(ctx);
    state.chat.set(chat);
    state.db.set(db);
}

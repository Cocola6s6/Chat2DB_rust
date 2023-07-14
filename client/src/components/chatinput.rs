use crate::components::chatoutput::Chatoutput;
use crate::models::chat::Chat;
use crate::models::db::Db;
use crate::AppState;
use std::mem::transmute;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use tracing::info;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlTextAreaElement};

// input 组件
#[component]
pub async fn Chatinput<G: Html>(ctx: Scope<'_>) -> View<G> {
    let state = use_context::<AppState>(ctx);
    let chat_ouput_signal = create_signal(ctx, String::from("AAA"));
    let db_ouput_signal = create_signal(ctx, String::from("BBB"));
    let query_tables_ouput_signal = create_signal(ctx, Vec::new());

    let window = web_sys::window().unwrap();
    let document = window.document().expect("no global document exists");

    let ask_btn: EventTarget = document.get_element_by_id("ask_btn").unwrap().into();
    button_event_listener(
        ctx,
        "click",
        ask_btn,
        Box::new(move || {
            spawn_local_scoped(ctx, async move {
                // 1、获取输入框内容
                let text = get_input_text();

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
        }),
    );

    let exec_btn: EventTarget = document.get_element_by_id("exec_btn").unwrap().into();
    button_event_listener(
        ctx,
        "click",
        exec_btn,
        Box::new(move || {
            spawn_local_scoped(ctx, async move {
                // 1、获取要执行的sql
                let sql = chat_ouput_signal.get().to_string();

                // 2、请求dbapi
                let resp = Db::exec_sql(state.db.get().db_url.clone(), sql)
                    .await
                    .unwrap_or_default();

                db_ouput_signal.set(resp);
            })
        }),
    );

    let query_tables_btn: EventTarget = document.get_element_by_id("query_tables_btn").unwrap().into();
    button_event_listener(
        ctx,
        "click",
        query_tables_btn,
        Box::new(move || {
            spawn_local_scoped(ctx, async move {
                // 请求dbapi
                let resp = Db::query_tables(state.db.get().db_url.clone(), state.db.get().db_ns.clone())
                    .await
                    .unwrap_or_default();
   
                    query_tables_ouput_signal.set(resp);
            })
        }),
    );

    view! {ctx,
        div {
            Chatoutput(chat_output_text=chat_ouput_signal, db_output_text=db_ouput_signal, tables_output_text=query_tables_ouput_signal)
        }
    }
}

// 监听按钮点击事件
pub fn button_event_listener<'a>(
    ctx: Scope<'_>,
    event: &str,
    button: EventTarget,
    callback: Box<dyn Fn() + 'a>,
) {
    let handler: Box<dyn Fn() + 'static> = unsafe { transmute(callback) };
    let callback = Closure::wrap(handler); // 使用 wasm Closure 可以将函数转为 JsValue

    button
        .add_event_listener_with_callback(event, callback.as_ref().unchecked_ref())
        .expect("监听请求发送失败");

    // on_cleanup 是一个 hooks 函数，当组件移除时触发
    on_cleanup(ctx, move || {
        info!("ctx on_cleanup]===================>");
        drop(callback);
    });
}

// 获取输入框内容
pub fn get_input_text() -> String {
    let window = web_sys::window().unwrap();
    let document = window.document().expect("no global document exists");
    let text = document
        .get_element_by_id("text")
        .unwrap()
        .dyn_into::<HtmlTextAreaElement>()
        .unwrap()
        .value();

    text
}

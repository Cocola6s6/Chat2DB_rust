use crate::components::chatoutput::Chatoutput;
use crate::AppState;
use anyhow::{Ok, Result};
use rand::Rng;
use std::mem::transmute;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use tracing::info;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{EventTarget, HtmlTextAreaElement, Request, RequestInit, RequestMode, Response};

// input 组件
#[component]
pub async fn Chatinput<G: Html>(ctx: Scope<'_>) -> View<G> {
    let ouput_signal = create_signal(ctx, String::from("AAA"));

    let window = web_sys::window().unwrap();
    let document = window.document().expect("no global document exists");

    let ask_btn: EventTarget = document.get_element_by_id("ask_btn").unwrap().into();
    let text = document
        .get_element_by_id("text")
        .unwrap()
        .dyn_into::<HtmlTextAreaElement>()
        .unwrap()
        .value();
    let text_signal = create_signal(ctx, String::from(text));
    let state = use_context::<AppState>(ctx);
    button_event_listener(
        ctx,
        "click",
        ask_btn,
        Box::new(move || {
            spawn_local_scoped(ctx, async move {
                let response = chat(
                    state.chat.get().openai_key.clone(),
                    state.sql.get().url.clone(),
                    state.sql.get().ns.clone(),
                    text_signal.get().to_string(),
                )
                .await
                .unwrap_or_default();
  
                ouput_signal.set(response.as_string().unwrap());
            })
        }),
    );

    view! {ctx,
        div {
            Chatoutput(output_text=ouput_signal)
        }
    }
}

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

pub async fn chat(
    openai_key: String,
    db_url: String,
    db_ns: String,
    text: String,
) -> Result<JsValue> {
    info!("[chat]======================>");
    // 创建Request请求
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    let str_json = format!(
        r#"
        {{
            "openai_key": "{}",
            "sql": {{
                "url": "{}",
                "ns": "{}"
            }},
            "text": "{}"
        }}
        "#,
        openai_key, db_url, db_ns, text
    ); // 注意类型一定要对应，否则会"400 BadRequest"
    opts.body(Some(&JsValue::from_str(str_json.as_str())));
    // opts.body(Some(&JsValue::from_str(text.as_str())));
    let url = format!("http://localhost:5000/chat/chatgpt");
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    // request.headers().set("Content-Type", "application/json");
    // request.headers().set("Accept", "application/json");

    // 使用web_sys调用window的api发送请求
    let window = web_sys::window()
        .ok_or("no windows exists".to_string())
        .unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    // 解析Response响应
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
    // let courses: Vec<Course> = json.into_serde().unwrap();
    info!("resp: {:?}", json);

    Ok(json.into())

    // let random_float: f64 = rand::thread_rng().gen();
    // Ok(random_float.to_string().into())
}

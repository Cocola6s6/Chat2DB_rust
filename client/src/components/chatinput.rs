use anyhow::Result;
use sycamore::futures::*;
use sycamore::prelude::*;
use tracing::info;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Event, HtmlInputElement, MouseEvent, Request, RequestInit, RequestMode, Response};

// chatinput 组件
#[component]
pub async fn Chatinput<G: Html>(ctx: Scope<'_>) -> View<G> {
    // let input_class = || {
    //     format!("w-full cursor-default rounded-md bg-white py-1.5 pl-3 pr-10 text-left text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 sm:text-sm sm:leading-6")
    // };

    let input_text = create_signal(ctx, "".to_string()); // input_text监听变量
    let output_text = create_signal(ctx, "".to_string()); // output_text监听变量
    let click_button = move |_| {
        info!("[click button]=======================>");
        // spawn_local_scoped(ctx, async move {
        //     let text = input_text.get();
        //     let response = chat(text.to_string()).await.unwrap_or_default();
        //     output_text.set(response.as_string().unwrap());
        // });
        let response = JsValue::from_str("hello");
        output_text.set(response.as_string().unwrap());
    };

    let set_output_text = create_memo(ctx, || {
        info!(
            "[set_output_text]=======================>{}",
            output_text.get()
        );
        output_text.get()
    });

    view! {ctx,
        form {
            div (class="grid gap-6 mb-6 md:grid-cols-2"){
                div (class="mb-6"){
                    label(class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"){"Input Here"}
                    input(type="text",
                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                    on:input = move |e: Event| {
                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                        let value = target.value();
                        info!("Input: {:?}", &value);
                        input_text.set(value);
                    }) {}
                }
                button(
                    class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800", type="submit",
                    on:click = click_button
                ) {
                    "Button"
                }
            }
        }
        p {
            "这里是输出的内容："
            (set_output_text.get())
        }
    }
}

pub async fn chat(text: String) -> Result<JsValue> {
    info!("[chat]======================>");
    // 创建Request请求
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    let str_json = format!(
        r#"
        {{
            "openai_key": "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",
            "sql": {{
                "url": "postgres://postgres:postgres@45.128.222.100:15432",
                "ns": "public"
            }},
            "text": "{}"
        }}
        "#,
        text
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

    Ok(json)
}

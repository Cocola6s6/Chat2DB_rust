use sycamore::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
    pub chat_output_text: &'a Signal<String>,
    pub db_output_text: &'a Signal<String>,
    pub tables_output_text: &'a Signal<Vec<String>>, 
}

// output 组件
#[component]
pub async fn Chatoutput<'a, G: Html>(ctx: Scope<'a>, props: Props<'a>) -> View<G> {
    create_memo(ctx, move || {
        // chat 内容展示
        let window = web_sys::window().unwrap();
        let document = window.document().expect("no global document exists");
        let chat_output = document.get_element_by_id("chat_output").unwrap();
        chat_output.set_inner_html(&props.chat_output_text.get());
    });

    create_memo(ctx, move || {
        // db 内容展示
        let window = web_sys::window().unwrap();
        let document = window.document().expect("no global document exists");
        let db_output = document.get_element_by_id("db_output").unwrap();
        db_output.set_inner_html(&props.db_output_text.get());
    });

    create_memo(ctx, move || {
        // db 内容展示
        let window = web_sys::window().unwrap();
        let document = window.document().expect("no global document exists");
        let tables_output = document.get_element_by_id("tables_output").unwrap();
        tables_output.set_inner_html(&props.tables_output_text.get().join("<br>"));
    });

    // TODO 考虑是否使用sycamore组件显示内容，现在是获取dom元素直接设置内容
    view! {ctx,
        // div {
        //     (*props.output_text.get())
        // }
    }
}

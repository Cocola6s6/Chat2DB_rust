use sycamore::prelude::*;
use tracing::info;

#[derive(Props)]
pub struct Props<'a> {
    pub chat_output_text: &'a Signal<String>,
    pub db_output_text: &'a Signal<String>,
    pub tables_output_text: &'a Signal<Vec<String>>,
}

// output 组件
#[component]
pub async fn Chatoutput<'a, G: Html>(ctx: Scope<'a>, props: Props<'a>) -> View<G> {
    // let chat_output_text = props.chat_output_text.get().to_string();
    // let db_output_text = props.db_output_text.get().to_string();
    // let tables_output_text = props.tables_output_text.get().join("<br>").to_string();

    let chat_output_text = create_memo(ctx, move || {
        let chat_output_text = props.chat_output_text.get().to_string();
        chat_output_text
    });

    let db_output_text = create_memo(ctx, move || {
        let db_output_text = props.db_output_text.get().to_string();
        db_output_text
    });

    let tables_output_text = create_memo(ctx, move || {
        let tables_output_text = props.tables_output_text.get().join("<br>").to_string();
        tables_output_text
    });

    view! {ctx,
        div{
            label(class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                (format!("CHAT_RESULT: {}", chat_output_text))
            }
            label(class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                (format!("DB_RESULT: {}", db_output_text))
            }
            // TODO 这里有问题，没有换行显示
            label(class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                (format!("TABLES_RESULT: {}", tables_output_text))
            }
        }
    }
}

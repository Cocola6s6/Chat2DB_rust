use std::collections::HashMap;

use sycamore::prelude::*;
use tracing::info;

#[derive(Props)]
pub struct Props<'a> {
    pub chat_output_text: &'a Signal<String>,
    pub db_output_text: &'a Signal<Vec<HashMap<String, String>>>,
    pub db_ouput_keys_text: &'a Signal<Vec<String>>,
    pub tables_output_text: &'a Signal<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cat {
    id: &'static str,
    name: &'static str,
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
        let db_output_text = props.db_output_text.get();
        db_output_text
    });

    let db_ouput_keys_text = create_memo(ctx, move || {
        let db_output_keys_text = props.db_ouput_keys_text.get().to_vec();
        db_output_keys_text
    });

    let tables_output_text = create_memo(ctx, move || {
        let tables_output_text = props.tables_output_text.get().join("<br>").to_string();
        tables_output_text
    });

    let items = create_signal(ctx, vec!["id", "name"]);

    let items_2 = create_signal(
        ctx,
        vec![
            [("name", "A"), ("id", "1")]
                .iter()
                .cloned()
                .collect::<std::collections::HashMap<_, _>>(),
            [("id", "2"), ("name", "B")]
                .iter()
                .cloned()
                .collect::<std::collections::HashMap<_, _>>(),
        ],
    );

    // let entry_signal = create_signal(ctx, HashMap::new());

    view! {ctx,
        div{
            label(class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                (format!("CHAT_RESULT: {}", chat_output_text))
            }

            // TODO 这里有问题
            label(class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                (format!("DB_RESULT: {}", chat_output_text))
            }
            // TODO 这里有问题，没有换行显示
            label(class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                (format!("TABLES_RESULT: {}", tables_output_text))
            }
        }

        div(class="relative overflow-x-auto"){
            table(class="w-full text-sm text-left text-gray-500 dark:text-gray-400"){
                thead(class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400"){
                    tr{
                        // 循环渲染表头数据
                        Indexed(
                            iterable=items,
                            view=|ctx, key| view! { ctx,
                                th(scope="col", class="px-6 py-3") {
                                    (key)
                                }
                            }
                        )
                    }
                }
                tbody(class="bg-white border-b dark:bg-gray-800 dark:border-gray-700") {
                    // 使用 Indexed 嵌套迭代渲染数据
                    Indexed(
                        iterable=items_2,
                        view=move |ctx, entry| {
                            // entry_signal.set(entry.clone());
                            view! { ctx,
                                tr(class="bg-white border-b dark:bg-gray-800 dark:border-gray-700") {
                                    // 使用 Indexed 迭代渲染单元格数据
                                    Indexed(
                                        iterable=items,
                                        view=move |ctx, key| view! { ctx,
                                            td(class="px-6 py-4") {
                                                // (entry_signal.get().get(key).unwrap())
                                                ("entry.get(key).unwrap()")
                                            }
                                        }
                                    )
                                }
                            }
                        }
                    )
                }
            }
        }
    }
}

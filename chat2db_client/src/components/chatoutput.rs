use std::collections::{HashMap, BTreeMap};

use sycamore::prelude::*;
use tracing::info;

#[derive(Props)]
pub struct Props<'a> {
    pub chat_output_text: &'a Signal<String>,
    pub db_output_text: &'a Signal<Vec<BTreeMap<String, String>>>,
    pub tables_output_text: &'a Signal<Vec<String>>,
}

// output 组件
#[component]
pub async fn Chatoutput<'a, G: Html>(ctx: Scope<'a>, props: Props<'a>) -> View<G> {
    // let chat_output_text = props.chat_output_text.get().to_string();
    // let db_output_text = props.db_output_text.get().to_string();
    // let tables_output_text = props.tables_output_text.get().join("<br>").to_string();

    let chat_output_text = create_memo(ctx, move || props.chat_output_text.get().to_string());
    let db_output_text = create_memo(ctx, move || props.db_output_text.get().to_vec());
    let db_output_keys_text = create_memo(ctx, move || {
        props
            .db_output_text
            .get()
            .get(0)
            .iter()
            .flat_map(|dict| dict.keys().cloned())
            .collect()
    });
    let tables_output_text = create_memo(ctx, move || {
        props.tables_output_text.get().to_vec()
    });

    let items = create_signal(ctx, vec!["id", "name"]);
    // let items_2 = create_signal(
    //     ctx,
    //     vec![
    //         [("name", "A"), ("id", "1")]
    //             .iter()
    //             .cloned()
    //             .collect::<std::collections::HashMap<_, _>>(),
    //         [("id", "2"), ("name", "B")]
    //             .iter()
    //             .cloned()
    //             .collect::<std::collections::HashMap<_, _>>(),
    //     ],
    // );

    view! {ctx,
        div{
            label(class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                (format!("CHAT_RESULT: {}", chat_output_text))
            }
        }

        div{
            label(class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"){
                ("Tables:")
            }
            select(class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"){
                Keyed(
                    iterable=tables_output_text,
                    view=|ctx, x| view! { ctx,
                        option{(x)}
                    },
                    key=|x| x.clone(),
                )
            }
        }

        div {
            label(class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                ("DB_RESULT:")
            }
    
            div(class="relative overflow-x-auto"){
                table(class="w-full text-sm text-left text-gray-500 dark:text-gray-400"){
                    thead(class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400"){
                        tr{
                            // 循环渲染表头数据
                            Indexed(
                                iterable=db_output_keys_text,
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
                            iterable=db_output_text,
                            view=move|ctx, entry| {
                                // TODO 注意：1）view中只能是dom元素，写方法会报错。2）内层view不能直接使用外层的变量。
                                // 需要通过信号量来通信
                                let entry_signal = create_signal(ctx, entry.clone());
                                view! { ctx,
                                    tr(class="bg-white border-b dark:bg-gray-800 dark:border-gray-700") {
                                        // 使用 Indexed 迭代渲染单元格数据
                                        Indexed(
                                            iterable=db_output_keys_text,
                                            view=move|ctx, key| {
                                                view! { ctx,
                                                    td(class="px-6 py-4") {
                                                        (entry_signal.get().get(&key).unwrap())
                                                    }
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
}

use std::collections::{BTreeMap};

use sycamore::prelude::*;
use tracing::info;

#[derive(Props)]
pub struct Props<'a> {
    pub db_output_text: &'a Signal<Vec<BTreeMap<String, String>>>,
    pub tables_output_text: &'a Signal<Vec<String>>,
}

// output 组件
#[component]
pub async fn Chatoutput<'a, G: Html>(ctx: Scope<'a>, props: Props<'a>) -> View<G> {
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
    let tables_output_text = create_memo(ctx, move || props.tables_output_text.get().to_vec());

    // let items = create_signal(ctx, vec!["id", "name"]);
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

        // table
        div(class="Dbtables h-[84px] left-[31px] top-[120px] absolute") {
            div(class="DbTables left-[18px] top-0 absolute text-black text-2xl font-medium font-['Poppins']") {
                "DB_Tables"
            }
            div(class="Frame425 w-[167px] h-[38px] p-2 left-0 top-[46px] absolute bg-white rounded-lg border border-zinc-300 justify-between items-center inline-flex") {
                div(class="Tables text-neutral-400 text-sm font-normal font-['Microsoft YaHei']") {
                    "tables"
                }
                select(class="Frame427319218 w-5 h-5 relative"){
                    Keyed(
                        iterable=tables_output_text,
                        view=|ctx, x| view! { ctx,
                            option{(x)}
                        },
                        key=|x| x.clone(),
                    )
                }
            }
        }

        // execResult
        div(class="Execresult w-[968px] h-[319px] left-[333px] top-[557px] absolute") {
            div(class="Openaikey w-[664.39px] h-[319px] left-[303.61px] top-0 absolute bg-white rounded border border-slate-300") {

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
                                    let entry_signal = create_signal(ctx, entry);
                                    view! { ctx,
                                        tr(class="bg-white border-b dark:bg-gray-800 dark:border-gray-700") {
                                            // 使用 Indexed 迭代渲染单元格数据
                                            Indexed(
                                                iterable=db_output_keys_text,
                                                view=move|ctx, key| {
                                                    view! { ctx,
                                                        td(class="px-6 py-4") {
                                                            (entry_signal.get().get(&key))
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

            div(class="Asktips w-[242.49px] h-[59.89px] left-[-0px] top-[44.27px] absolute") {
                div( class="ResultHere w-[176.48px] h-[21.63px] left-[-0px] top-[33.27px] absolute text-black text-2xl font-normal font-['Open Sans'] leading-[14px]") {
                    "Result Here"
                }
                div(class=" w-[29.64px] h-[59.89px] left-[212.85px] top-[-0px] absolute text-center text-black text-4xl font-normal font-['Open Sans'] leading-9") {
                    "👋"
                }
            }
        }

    }
}

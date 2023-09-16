use sycamore::prelude::*;
use sycamore_router::{Route, Router, RouterProps};
use tracing::info;

use crate::{components::{chatinput, connection}, models::db, AppState};

// TODO 解决路由参数传递问题
#[derive(Props)]
pub struct Props<'a> {
    pub app_state: &'a Signal<AppState>,
}

#[derive(Debug, Route)]
pub enum Routes {
    #[to("/connection")]
    Connection,
    #[to("/chatinput")]
    Chatinput,
    #[not_found]
    NotFound,
}

#[component]
pub fn switch<'a, G: Html>(ctx: Scope<'a>, route: &'a ReadSignal<Routes>) -> View<G> {
    info!("[router switch]====================>");
    let state = use_context::<AppState>(ctx);
    info!("state={:?}=======================>", state);

    let openai_key_text = create_signal(ctx, String::from(""));
    let db_url_text = create_signal(ctx, String::from(""));
    let db_ns_text = create_signal(ctx, String::from(""));

    let view = create_memo(
        ctx,
        on([route], move || match route.get().as_ref() {
            Routes::Connection => view! { ctx,
                connection::Connection
            },
            Routes::Chatinput => view! { ctx,
                chatinput::Chatinput(
                    openai_key_text=openai_key_text,
                    db_url_text=db_url_text,
                    db_ns_text=db_ns_text,
                )
            },
            Routes::NotFound => view! { ctx,
                "404 Not Found"
            },
        }),
    );
    
    view! { ctx,
        div {
            ((*view.get()))
        }
    }
}

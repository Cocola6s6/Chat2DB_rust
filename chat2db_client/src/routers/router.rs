use sycamore::prelude::*;
use sycamore_router::{Route, Router, RouterProps};
use tracing::info;

use crate::{components::{chatinput, connection}, models::db, AppState};


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

    let view = create_memo(
        ctx,
        on([route], move || match route.get().as_ref() {
            Routes::Connection => view! { ctx,
                connection::Connection
            },
            Routes::Chatinput => view! { ctx,
                chatinput::Chatinput
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

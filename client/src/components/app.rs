use crate::components::chatinput::Chatinput;
use sycamore::prelude::*;

// App 组件
#[component]
pub async fn App<G: Html>(ctx: Scope<'_>) -> View<G> {
    view! {ctx,
        div {
            p {
                "Hello, world!"
            }
            div {
                Chatinput()
            }
        }
    }
}

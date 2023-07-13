use sycamore::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
    pub output_text: &'a Signal<String>,
}

// output 组件
#[component]
pub async fn Chatoutput<'a, G: Html>(ctx: Scope<'a>, props: Props<'a>) -> View<G> {

    view! {ctx,
        div {
            (*props.output_text.get())
        }
    }
}
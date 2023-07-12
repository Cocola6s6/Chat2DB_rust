use sycamore::prelude::*;
use client::components::app::App;

fn main() {
    println!("Hello, world!");

    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    sycamore::render(|ctx| view!(ctx, App))
}

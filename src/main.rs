use dioxus::prelude::*;

mod components;

use components::carousel::Carousel;

fn main() {
    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            class: "container mx-auto",
            Carousel {}
        }
    ))
}

use dioxus::prelude::*;

pub fn Carousel(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            class: "shadow-lg shadow-black",
            h1 {
                class: "text-4xl py-4",
                "From Library"
            },
            div {
                class: "flex justify-around",
                CarouselBook {}
            }
        }
    ))
}

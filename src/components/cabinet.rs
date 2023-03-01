use crate::components::shelf::Shelf;
use dioxus::prelude::*;

//TODO: Replace placeholder text with fill logic.
pub fn BookCase(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "",
            div {
                h1 {
                    class: "text-4xl font-bold",
                    "By Placeholder(this case by tag)"
                }
            },
            div {
                class: "grid grid-cols-2 gap-4",
                Shelf {},
                Shelf {}
            }
        }
    ))
}

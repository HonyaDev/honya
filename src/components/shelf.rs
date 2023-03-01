use crate::components::book::Book;
use dioxus::prelude::*;

//TODO: Replace placeholder text with actual fill logic
pub fn Shelf(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            div {
                class: "",
                div {
                    class: "shelf grid shadow-lg shadow-black rounded",
                    h3 {
                        class: "text-lg font-semibold pl-4",
                        "Survival Horror(Placeholder Tag)"
                    },
                    div {
                        class: "grid gap-4 p-4",
                        Book {},
                        Book {}
                    },
                }
            }
        },
    ))
}

use dioxus::prelude::*;

//TODO: Replace placeholder text with actual logic
pub fn Book(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "books flex gap-3",
            img {
                class: "rounded row-span-3",
                src: "https://i.pinimg.com/474x/33/dd/6b/33dd6b3426fb61188d1e43566fc9ec6b.jpg",
                alt: "Spongebob mocking",
                width: "150",
                height: "120",
            },
            div {
                div {
                    class: "flex",
                    div {
                        "🚩"
                    },
                    div {
                        class: "col-span-6",
                        "title: Placeholder"
                    },
                },
                div {
                    class: "author",
                    "Author: Placeholder"
                },
                div {
                    class: "group",
                    "Group: Placeholder"
                },
                div {
                    class: "series",
                    "Series: Placeholder"
                },
                div {
                    class: "row-span-2 col-span-6",
                    "tags: tag1, tag2, tag3"
                }
            },
        }
    ))
}

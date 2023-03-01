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
                for _ in 0..3 {
                    CarouselBook {}
                }
            }
        }
    ))
}

//TODO: Mock component, replace
fn CarouselBook(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            img {
                class: "rounded",
                src: "https://i.pinimg.com/474x/33/dd/6b/33dd6b3426fb61188d1e43566fc9ec6b.jpg",
                alt: "Spongebob mocking",
                width: "200",
                height: "200",
            },
            h4 {
                class: "text-lg font-bold text-center",
                "Title: Test Book"
            }
        }
    ))
}

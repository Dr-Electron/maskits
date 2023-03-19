#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "hello, wasm!" }
        div {
            input { placeholder: "What needs to be done?" }
            button { "Generate Random 24 Words" }
        }
        img { src: "https://upload.wikimedia.org/wikipedia/en/7/7d/Lenna_%28test_image%29.png" }
    })
}

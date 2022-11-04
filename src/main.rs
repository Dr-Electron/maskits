// main.rs

use dioxus::{prelude::{*, dioxus_elements::{button, img}}, events::onclick};

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!{
        div { "hello, wasm!" }
        div {
            input { placeholder: "What needs to be done?" }
            button { "Generate Random 24 Words" }
        }
        img { src: "https://upload.wikimedia.org/wikipedia/en/7/7d/Lenna_%28test_image%29.png" }
    })
}

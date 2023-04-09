// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use iota_client::Client;

use crate::nft::mint_nft;

mod nft;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let client = Client::builder()
        .with_node("https://api.testnet.shimmer.network")
        .unwrap()
        .finish()
        .unwrap();

    // Get node info.
    let info = client.get_info().await.unwrap();

    log::info!("{info:#?}");

    // launch the web app
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" },
        header {
            img {
                src: "assets/hero.svg"
            }
        }
        div {
            class: "flex flex-col items-center py-16",
            background_color: "#9DA982",
            div {
                class: "sm:w-4/6",
                div {
                    class: "text-white px-12 mb-12",
                    div {
                        class: "flex font-semibold border-b-2 mb-4",
                        border_color: "#B5C299",
                        div {
                            class: "px-4 py-1",
                            background_color: "#B5C299",
                            "STEP 1"
                        }
                        div {
                            class: "flex-1 px-4 py-1",
                            "Lorem Ipsum"
                        }
                    }
                    div {
                        "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet."
                    }
                }
                button {
                    class: "bg-white font-semibold rounded-md w-full py-4",
                    color: "#838383",
                    "Lorem Ipsum"
                }
            }
        }
        div {
            class: "flex flex-col items-center py-16",
            background_color: "#5B6050",
            div {
                class: "sm:w-4/6",
                div {
                    class: "text-white px-12 mb-12",
                    div {
                        class: "flex font-semibold border-b-2 mb-4",
                        border_color: "#B5C299",
                        div {
                            class: "px-4 py-1",
                            background_color: "#B5C299",
                            "STEP 2"
                        }
                        div {
                            class: "flex-1 px-4 py-1",
                            "Customize your NFT character and mint it"
                        }
                    }
                    div {
                        "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet."
                    }
                }
                div {
                    class: "flex gap-4 bg-white rounded-md p-12",
                    color: "#838383",
                    div {
                        class: "flex-1",
                        img {
                            src: "assets/avatar_bg.svg"
                        }
                    }
                    div {
                        class: "flex flex-col gap-4 flex-none w-1/4",
                        div {
                            class: "flex-1"
                        }
                        button {
                            class: "text-white font-semibold w-full py-4 border-4",
                            border_color: "#DE7676",
                            background_color: "#DE7676",
                            "RANDOMIZE"
                        }
                        button {
                            class: "text-white font-semibold w-full py-4 border-4",
                            border_color: "#A5B08D",
                            background_color: "#B5C299",
                            onclick: move |_| mint_nft(),
                            "MINT"
                        }
                    }
                }
            }
        }
        footer {
            class: "h-40",
            background_color: "#282A23"
        }
    })
}

#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::{info, Level};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        img { src: "header.svg", id: "header" }
        div { id: "links",
            a { href: "https://dioxuslabs.com/learn/0.5/", "📚 Learn Dioxus" }
            a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
            a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
            a { href: "https://github.com/DioxusLabs/dioxus-std", "⚙️ Dioxus Standard Library" }
            a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                "💫 VSCode Extension"
            }
            a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}

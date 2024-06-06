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

    let mut modal_visible = use_signal(|| false);
    let modal_cb = use_callback(move || modal_visible.set(false));

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
        button {
            class: "px-4 py-2 focus:outline-none focus-visible:ring focus-visible:ring-green-400 font-semibold text-base bg-green-800 text-amber-50 rounded-full shadow-lg",
            onclick: move |_| modal_visible.set(true),
            "Show Modal"
        }
        if modal_visible() {
            Modal {
                on_dismiss: modal_cb,
                div {
                    class: "p-2 sm:p-4 mx-auto my-2 flex flex-col items-center gap-4 text-center",
                    h1 {
                        class: "text-2xl",
                        "Hi!"
                    }
                    div {
                        class: "text-lg",
                        "This is a modal!"
                    }
                    button {
                        class: "px-4 py-2 focus:outline-none focus-visible:ring focus-visible:ring-green-400 font-semibold text-base bg-green-800 text-amber-50 rounded-full shadow-lg",
                        onclick: move |_| {
                            modal_visible.set(false);
                        },
                        "Close Modal"
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct ModalProps {
    pub children: Element,
    pub extra_backdrop_classes: Option<String>,
    pub extra_modal_classes: Option<String>,
    /// If dismissable, an X button will be shown.
    /// If dismissable, clicking the X button or backdrop will close the modal.
    #[props(default = true)]
    pub dismissable: bool,
    pub on_dismiss: Option<UseCallback<()>>,
}

pub fn Modal(props: ModalProps) -> Element {
    let ModalProps {
        children,
        extra_backdrop_classes,
        extra_modal_classes,
        dismissable,
        on_dismiss,
    } = props;

    let mut dismissed = use_signal(|| false);

    let mut on_dismiss_handler = move |_| {
        if dismissable {
            dismissed.set(true);
            if let Some(on_dismiss) = on_dismiss {
                on_dismiss.call();
            }
        }
    };

    rsx! {
        div {
            class: "fixed inset-0 z-20 block backdrop-blur-sm bg-amber-50/20 overflow-hidden transition-[opacity,backdrop-blur] duration-1000",
            class: if let Some(extra_backdrop_classes) = extra_backdrop_classes {
                extra_backdrop_classes
            },
            class: if dismissed() {
                "invisible z-[-1] opacity-0 backdrop-blur-none"
            },
            onclick: move |e| {
                // tracing::debug!("Modal background pressed; dismissing!");
                on_dismiss_handler(e);
            },
            div {
                class: "w-full h-auto bottom-0 absolute bg-amber-50 absolute animate-slide-up",
                class: if let Some(extra_modal_classes) = extra_modal_classes {
                    extra_modal_classes
                },
                onclick: |e| {
                    // Don't close modal when interacting with its content
                    // tracing::debug!("Modal content click event; stopping propagation!");
                    e.stop_propagation();
                },
                button {
                    class: "absolute top-0 right-0 p-2",
                    class: if dismissable {
                        "visible"
                    } else {
                        "invisible"
                    },
                    onclick: move |e| {
                        // tracing::debug!("Modal button pressed; dismissing!");
                        on_dismiss_handler(e);
                    },
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke_width: "1.5",
                        class: "w-6 h-6",
                        path {
                            stroke_linejoin: "round",
                            d: "M6 18 18 6M6 6l12 12",
                            stroke_linecap: "round"
                        }
                    }

                }
                {children}
            }
        }
    }
}

#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

const L_SIDEBAR_W: u32 = 48;
const R_SIDEBAR_W: u32 = 48;
const NAVBAR_H: u32 = 16;

mod svg_icon;
mod tree;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "tailwind.css" }
        Column {}
    }
}

#[component]
fn Column() -> Element {
    rsx! {
        div { class: "flex min-h-screen",
            aside { class: format!("sticky top-0 h-[calc(100vh-theme(spacing.0))] w-{} overflow-y-auto bg-green-200", L_SIDEBAR_W),
                DrawerLeft {}
            }
            main { class: format!("flex-1 mt-{} left-{} right-{} bg-yellow-200", NAVBAR_H, L_SIDEBAR_W, R_SIDEBAR_W),
                Cells {}
                nav { class: format!("fixed h-{} w-full top-0 bg-blue-200", NAVBAR_H),
                    "Nav"
                }
            }
            aside { class: format!("sticky top-0 h-[calc(100vh-theme(spacing.0))] w-{} right-0 overflow-y-auto", R_SIDEBAR_W),
                DrawerRight {}
            }

        }
    }
}

#[component]
fn DrawerLeft() -> Element {
    rsx! {
        ul { class: "menu w-full min-h-full bg-white text-base-content",
            li {
                a { "Sidebar Item 1" }
            }
            li {
                a { "Sidebar Item 2" }
            }
        }
    }
}

#[component]
fn DrawerRight() -> Element {
    rsx! {

        ul { class: "menu p-4 right-0 w-48 min-h-full bg-white text-base-content",
            li {
                a { "Menu Item A" }
            }
            li {
                a { "Menu Item B" }
            }
        }

    }
}

#[component]
fn Cells() -> Element {
    rsx! {
        div { class: "flex flex-col p-6 items-center bg-base-200",
            for _i in 0..12 {
                Cell {}
                // div { class: "divider divider-neutral", "next" }
            }
        }
    }   
}

#[component]
fn Cell() -> Element {
    rsx! {
        div { class: "w-2/3 my-2 px-6 py-2 rounded-lg bg-white shadow",
            div { class: "text-gray-400",
                "2024/05/25 23:41"
            }
            div { class: "flex divide-x",
                span { class: "w-full h-full flex justify-center items-center mr-4",
                    "集りはゴーシュのきょろきょろ狸みちの扉をおろし音だです。そこでどっかり生ましましって小太鼓でまし。それどころたますんましはでしまたおばあさんの愉快目のうちがもまるで変だろまして、われわれじゃ小節に叫ぶがっ方たた。"
                }
                // div {
                //     Fileprops {}
                // }
                div { class: "min-w-60",
                    tree::tree {}
                }
            }
        }
    }
}

#[component]
fn Fileprops() -> Element {
    rsx! {
        div { class: "overflow-x-auto",
            table { class: "table",
                thead {
                    tr {
                        th { "" }
                    }
                }
                for _i in 0..1 {
                    tbody {
                        tr {
                            td {
                                div { class: "flex items-center gap-3",
                                    div { class: "avatar",
                                        div { class: "mask mask-squircle w-6 h-6",
                                            svg_icon::directory {}
                                        }
                                    }
                                    div { "Directory_name"}
                                }
                            }
                        }
                    }
                }
                for _i in 0..3 {
                    tbody {
                        tr {
                            td {
                                div { class: "btn flex items-center gap-3",
                                    div { class: "avatar",
                                        div { class: "mask mask-squircle w-6 h-6",
                                            svg_icon::document {}
                                        }
                                    }
                                    div { "File_name_long__________.docx"}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

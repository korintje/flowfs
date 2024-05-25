use dioxus::prelude::*;

pub fn document() -> Element {
    rsx! {
        svg {
            width: "24px",
            "viewBox": "0 -960 960 960",
            height: "24px",
            "xmlns": "http://www.w3.org/2000/svg",
            "fill": "#5f6368",
            path { "d": "M320-240h320v-80H320v80Zm0-160h320v-80H320v80ZM240-80q-33 0-56.5-23.5T160-160v-640q0-33 23.5-56.5T240-880h320l240 240v480q0 33-23.5 56.5T720-80H240Zm280-520v-200H240v640h480v-440H520ZM240-800v200-200 640-640Z" }
        }
    }
}

pub fn directory() -> Element {
    rsx! {
        svg {
            "xmlns": "http://www.w3.org/2000/svg",
            height: "24px",
            "viewBox": "0 -960 960 960",
            "fill": "#5f6368",
            width: "24px",
            path { "d": "M160-160q-33 0-56.5-23.5T80-240v-480q0-33 23.5-56.5T160-800h240l80 80h320q33 0 56.5 23.5T880-640v400q0 33-23.5 56.5T800-160H160Zm0-80h640v-400H447l-80-80H160v480Zm0 0v-480 480Z" }
        }
    }
}
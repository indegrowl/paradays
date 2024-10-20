use dioxus::prelude::*;

use crate::theme::Theme;

pub fn Sidebar() -> Element {
    let theme_context = use_context::<Signal<Theme>>();
    let mut is_open = use_signal(|| false);

    rsx! {
        nav {
            class:"fixed top-0 left-0 p-2 visible lg:invisible border-b w-full",
            style: "border-color: {theme_context().border_color};",
            svg {
                "stroke-width": "2",
                "stroke": "{theme_context().text_color}",
                width: "24",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                "viewBox": "0 0 24 24",
                "xmlns": "http://www.w3.org/2000/svg",
                "fill": "none",
                height: "24",
                onclick: move |_| {is_open.set(true)},
                line {
                    "x2": "20",
                    "x1": "4",
                    "y1": "12",
                    "y2": "12"
                }
                line {
                    "y1": "6",
                    "x2": "20",
                    "y2": "6",
                    "x1": "4"
                }
                line {
                    "y1": "18",
                    "x2": "20",
                    "y2": "18",
                    "x1": "4"
                }
            },
        }
        aside {
            style: "border-color: {theme_context().border_color}; background-color: {theme_context().background_color}",
            class:format!("fixed top-0 left-0 z-40 w-64 h-screen transition-transform {} lg:translate-x-0 border-r", if !is_open() {
                "-translate-x-full"
            } else {
                "-translate-x-0"
            })
        }
        div {
            onclick: move |_| {is_open.set(false)},
            class:format!("fixed bg-black top-0 left-0 z-30 w-screen h-screen transition-opacity {}", if !is_open() {
                "opacity-0 invisible"
            } else {
                "opacity-30 visible"
            })

        }
    }
}

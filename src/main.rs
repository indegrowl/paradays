#![allow(non_snake_case)]

use components::editor::Editor;
use components::sidebar::Sidebar;

use dioxus::prelude::*;
use dioxus_logger::tracing;
use theme::Theme;

mod components;
mod theme;

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    let default_theme = use_signal(move || {
        Theme {
            primary_color: "#8aadf4",    // Blue (Primary)
            secondary_color: "#f5bde6",  // Pink (Secondary)
            background_color: "#24273a", // Dark Background
            text_color: "#cad3f5",       // Light Text Color
            accent_color: "#f28fad",     // Red Accent
            border_color: "#494d64",     // Dark Gray Border
        }
    });

    use_context_provider(move || default_theme);

    rsx! {
        div {
            class: "w-screen h-screen flex p-0 m-0",
            style: "background-color:{default_theme().background_color};color:{default_theme().text_color}",
            Sidebar {}
            Editor {}
        }
    }
}

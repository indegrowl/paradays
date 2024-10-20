use dioxus::prelude::*;

use crate::theme::Theme;

pub fn Editor() -> Element {
    let theme_context = use_context::<Signal<Theme>>();

    rsx! {
        div {
            class: "w-full p-6 pt-12 lg:ml-64 overflow-auto",
            style: "background-color: {theme_context().background_color}",
            for i in 0..15 {
                p {
                    class: "py-2 focus:outline-none",
                    key: "{i}",
                    contenteditable: true,
                    "Lorem ipsum odor amet, consectetuer adipiscing elit. Diam donec iaculis magna ullamcorper nullam. Volutpat porta neque mattis integer risus nostra nec enim in. At pellentesque vel mollis integer dictum auctor pharetra. Amet curae lorem sodales accumsan praesent iaculis. Mattis potenti suscipit vestibulum in massa posuere sociosqu. Ultrices libero volutpat mus sodales mauris non commodo nibh."
                }
            }
        }
    }
}

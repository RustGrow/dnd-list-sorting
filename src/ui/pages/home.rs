#![allow(non_snake_case)]
use crate::ui::components::nav_bar::NavBar;
use dioxus::prelude::*;

#[component]
pub fn Home(cx: Scope) -> Element {
    render! {
        section { class: "flex flex-row",
            NavBar {}
            div { class: "h-screen w-full bg-[#0b0423] text-center align-middle text-white",
                "Home Page"
            }
        }
    }
}

#![allow(non_snake_case)]
use crate::ui::components::side_bar::SideBar;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        section { class: "flex flex-row",
            SideBar {}
            div { class: "h-screen w-full bg-[#0b0423] text-center align-middle text-white",
                "Home Page"
            }
        }
    }
}

#![allow(non_snake_case)]
use crate::route::Route;
use dioxus::prelude::*;

pub fn SideBar() -> Element {
    rsx! {
        div { class: "w-64 h-screen bg-slate-400",
            div { class: "h-20" }
            ul { class: "",
                li {
                    Link {
                        class: "w-full flex flex-col items-center border-2 p-4 rounded hover:bg-slate-300 hover:rounded-xl",
                        to: Route::Home {},
                        span { "Home" }
                    }
                }
                li {
                    Link {
                        class: "w-full flex flex-col items-center border-2 p-4 rounded hover:bg-slate-300 hover:rounded-xl",
                        to: Route::DndLists {},
                        span { "Dnd with lists" }
                    }
                }
            }
        }
    }
}

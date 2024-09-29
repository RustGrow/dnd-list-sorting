#![allow(non_snake_case)]
use dioxus::prelude::*;

#[allow(unused)]
#[component]
pub fn Err404(segments: Vec<String>) -> Element {
    rsx! {
        section {
            div { class: "p-5 bg-slate-500", "Error404" }
        }
    }
}

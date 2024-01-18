#![allow(non_snake_case)]
use dioxus::prelude::*;

#[allow(unused)]
#[component]
pub fn Err404(cx: Scope, segments: Vec<String>) -> Element {
    render! {
        section { div { class: "p-5 bg-slate-500", "Error404" } }
    }
}

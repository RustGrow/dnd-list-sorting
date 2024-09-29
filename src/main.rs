#![allow(non_snake_case)]
mod models;
pub mod route;
mod ui;
use crate::route::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use models::{app_state::ApplicationData, item::Card};

pub const STYLE: &str = asset!("./assets/tailwind.css");

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

pub fn App() -> Element {
    let colors = vec!["bg-red-500", "bg-blue-500", "bg-green-500", "bg-yellow-500"];
    let tasks_list: Signal<Vec<Card>> = use_signal(|| {
        colors
            .iter()
            .enumerate()
            .map(|(id, color)| Card::new(id, 0, "Left List", color))
            .collect()
    });
    let end_tasks_list: Signal<Vec<Card>> = use_signal(|| {
        colors
            .iter()
            .enumerate()
            .map(|(id, color)| Card::new(id, 1, "Right List", color))
            .collect()
    });

    let board: Signal<Vec<Signal<Vec<Card>>>> = use_signal(|| vec![tasks_list, end_tasks_list]);
    use_context_provider(|| ApplicationData::new(board));

    rsx! {
        head::Link { rel: "stylesheet", href: STYLE }
        Router::<Route> {}
    }
}

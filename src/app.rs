#![allow(non_snake_case)]
use crate::models::app_state::ApplicationData;
use crate::models::item::ItemCard;
use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_signals::*;

pub fn App(cx: Scope) -> Element {
    let colors = vec!["bg-red-500", "bg-blue-500", "bg-green-500", "bg-yellow-500"];
    let tasks_list: Signal<Vec<ItemCard>> = use_signal(cx, || {
        colors
            .iter()
            .enumerate()
            .map(|(id, color)| ItemCard::new(id, 0, "Left Card", color))
            .collect()
    });
    let end_tasks_list: Signal<Vec<ItemCard>> = use_signal(cx, || {
        colors
            .iter()
            .enumerate()
            .map(|(id, color)| ItemCard::new(id, 1, "Right List", color))
            .collect()
    });

    let board: Signal<Vec<Signal<Vec<ItemCard>>>> =
        use_signal(cx, || vec![tasks_list, end_tasks_list]);
    use_context_provider(cx, || ApplicationData::new(board));

    // tracing_wasm::set_as_global_default_with_config(
    //     tracing_wasm::WASMLayerConfigBuilder::new()
    //         .set_max_level(tracing::Level::TRACE)
    //         .build(),
    // );

    render! { Router::<Route> {} }
}

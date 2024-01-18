use super::item::ItemCard;
use dioxus::prelude::*;
use dioxus_signals::*;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct ApplicationData {
    pub board: Signal<Vec<Signal<Vec<ItemCard>>>>,
}

impl ApplicationData {
    pub fn new(board: Signal<Vec<Signal<Vec<ItemCard>>>>) -> Self {
        Self { board }
    }

    pub fn use_app_data(cx: &ScopeState) -> ApplicationData {
        *use_context(cx).unwrap()
    }
}

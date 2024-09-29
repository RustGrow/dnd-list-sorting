#![allow(non_snake_case)]
use crate::models::item::Card;
use crate::ui::components::item_card::ItemCardUi;
use dioxus::prelude::*;

#[component]
pub fn Divider() -> Element {
    rsx! {
        div { class: "h-5" }
    }
}

#[component]
pub fn Button(
    mut list: Signal<Vec<Card>>,
    list_id: usize,
    click: fn(mylist: Signal<Vec<Card>>, list_id: usize),
    text: &'static str,
) -> Element {
    rsx! {
        div {
            class: "flex-0 p-2 text-white text-center bg-red-600 hover:bg-red-500 rounded-md cursor-pointer",
            onclick: move |_| { click(list, list_id) },
            "{text}"
        }
    }
}

pub fn InsertAboveCard(mut list: Signal<Vec<Card>>, list_id: usize) {
    list.write()
        .insert(0, Card::new(77, list_id, "Above card", "bg-red-600"));
}
pub fn InsertBottomCard(mut list: Signal<Vec<Card>>, list_id: usize) {
    list.write()
        .push(Card::new(88, list_id, "Bottom Card", "bg-red-600"));
}

#[component]
pub fn ListBox(
    box_name: &'static str,
    list_id: usize,
    list: Signal<Vec<Signal<Vec<Card>>>>,
    dragStartState: Signal<usize>,
    listIdInCard: Signal<usize>,
) -> Element {
    rsx! {
        div { class: "text-white text-center bg-red-600 py-2", "{box_name}" }
        ul { class: "w-64 border-2 border-dashed text-white", id: "{list_id}",
            {
                list.read()[list_id].read().iter().enumerate().map(|(id, card)| {
                rsx!{
                    ItemCardUi  {
                        card: *card,
                        id: id,
                        list_id: list_id,
                        color: card.color,
                        lists: list,
                        dragStartState: dragStartState,
                        listIdInCard: listIdInCard,
                    }
                }
            })

            }
        }
    }
}

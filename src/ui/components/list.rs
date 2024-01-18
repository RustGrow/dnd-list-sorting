#![allow(non_snake_case)]
use crate::models::item::ItemCard;
use crate::ui::components::item_card::ItemCardUi;
use dioxus::prelude::*;
use dioxus_signals::*;

#[component]
pub fn Divider(cx: Scope) -> Element {
    render! { div { class: "h-5" } }
}

#[component]
pub fn Button(
    cx: Scope,
    list: Signal<Vec<ItemCard>>,
    list_id: i32,
    click: fn(mylist: Signal<Vec<ItemCard>>, list_id: usize),
    text: &'static str,
) -> Element {
    render! {
        div {
            class: "flex-0 p-2 text-white text-center bg-red-600 hover:bg-red-500 rounded-md cursor-pointer",
            onclick: move |_| { click(*list, *list_id as usize) },
            "{text}"
        }
    }
}

pub fn InsertAboveCard(list: Signal<Vec<ItemCard>>, list_id: usize) {
    list.write()
        .insert(0, ItemCard::new(77, list_id, "Above card", "bg-red-600"));
}
pub fn InsertBottomCard(list: Signal<Vec<ItemCard>>, list_id: usize) {
    list.write()
        .push(ItemCard::new(88, list_id, "Bottom Card", "bg-red-600"));
}

#[component]
pub fn ListBox(
    cx: Scope,
    box_name: &'static str,
    list_id: usize,
    list: Signal<Vec<Signal<Vec<ItemCard>>>>,
    dragStartState: Signal<usize>,
    listIdInCard: Signal<usize>,
) -> Element {
    render! {
        div { class: "text-white text-center bg-red-600 py-2", "{box_name}" }
        ul { class: "w-64 border-2 border-dashed text-white", id: "{list_id}",
            {
                                list.read()[*list_id].read().iter().enumerate().map(|(id, card)| {
                            render!{
                                ItemCardUi  {
                                    card: *card,
                                    id: id,
                                    list_id: *list_id as usize,
                                    color: card.color,
                                    signal: *list,
                                    dragStartState: *dragStartState,
                                    listIdInCard: *listIdInCard,
                                }
                            }
                            })

                            }
        }
    }
}

#![allow(non_snake_case)]
// use crate::models::app_state::ApplicationData;
use crate::models::item::Card;
use crate::ui::components::icons::{ListBuller, TrashOutline};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn ItemCardUi(
    card: Card,
    id: usize,
    list_id: usize,
    color: &'static str,
    mut lists: Signal<Vec<Signal<Vec<Card>>>>,
    dragStartState: Signal<usize>,
    listIdInCard: Signal<usize>,
) -> Element {
    let mut firstOver = use_signal(|| false);
    let mut over = use_signal(|| false);
    // let data = use_context::<ApplicationData>();

    rsx! {
        li {
            match id {
                0 => rsx!{
                    div {
                    class: "w-full hover:scale-105 transform transition duration-200",
                    class: if *firstOver.read() {"h-14 bg-slate-600 border-2 border-dashed"} else {"h-8 bg-slate-300"},

                    ondragover: move |ev| {
                        ev.prevent_default();
                        *firstOver.write() = true},
                    ondragleave: move |_| *firstOver.write() = false,

                    ondrop: move |ev| {
                        ev.prevent_default();
                    *firstOver.write() = false;
                    let cc = dragStartState();
                    info!("DropToID: {id:?} and dragStartStateID {cc}");
                    // We don't need to change first card
                    //    if card.board_list_id == *list_id {
                    if dragStartState() != 0 || dragStartState() == 0 && listIdInCard() != list_id {
                    if listIdInCard() == list_id {
                    let item = lists()[listIdInCard()]
                        .write()
                        .remove(dragStartState());
                        lists()[list_id].write().insert(id, item);
                        } else {
                    let mut item = lists()[listIdInCard()]
                        .write()
                        .remove(dragStartState());
                        item.board_list_id = list_id;
                        lists()[list_id].write().insert(id, item);
                        }
                        }

                        },
                        "{id}"
                        }

                        },
                _ => rsx!{},
            },
            div {
                class: "list h-14 text-rose-100 cursor-move flex flex-row items-center justify-between gap-3 opacity-100 {color}",
                draggable: true,
                onclick: move |event| {
                    info!("Clicked! Event: {event:?}");
                },
                ondragstart: move |_| {
                    *dragStartState.write() = id;
                    let cc = dragStartState();
                    let card_board_list_id = card.board_list_id;
                    *listIdInCard.write() = card.board_list_id;
                    info!(
                        "DragStartID! {id:?} StateCardID {cc:?} dragstartListINCARD-{card_board_list_id} dragStartLIST-{list_id}"
                    );
                },
                div { ListBuller {} }
                "{card.title} {id}"
                div {
                    onclick: move |_| {
                        lists()[list_id].write().remove(id);
                    },
                    TrashOutline {}
                }
            }
            div {
                class: "w-full hover:scale-105 transform transition duration-200",
                class: if *over.read() {
                    "h-14 bg-slate-600 border-2 border-dashed"
                } else {
                    "h-8 bg-slate-300"
                },
                ondragover: move |ev| {
                    ev.prevent_default();
                    *over.write() = true;
                },
                ondragleave: move |_| *over.write() = false,
                ondrop: move |ev| {
                    ev.prevent_default();
                    *over.write() = false;
                    let cc = *dragStartState.read();
                    info!("DropListID-{list_id} DropID: {id:?} dragStartState {cc}");
                    if *listIdInCard.read() == list_id {
                        if *dragStartState.read() == id {} else {
                            if *dragStartState.read() < id {
                                let item = lists()[list_id].write().remove(*dragStartState.read());
                                lists()[list_id].write().insert(id, item);
                            } else {
                                let item = lists()[list_id].write().remove(*dragStartState.read());
                                lists()[list_id].write().insert(id + 1, item);
                            }
                        }
                    } else {
                        let mut item = lists()[*listIdInCard.read()]
                            .write()
                            .remove(*dragStartState.read());
                        item.board_list_id = list_id;
                        lists()[list_id].write().insert(id + 1, item);
                    }
                },
                "{id}"
            }
        }
    }
}

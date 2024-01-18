#![allow(non_snake_case)]
use crate::models::app_state::ApplicationData;
use crate::models::item::ItemCard;
use dioxus::prelude::*;
use dioxus_signals::*;

#[component]
pub fn ItemCardUi(
    cx: Scope,
    card: ItemCard,
    id: usize,
    list_id: usize,
    color: &'static str,
    signal: Signal<Vec<Signal<Vec<ItemCard>>>>,
    dragStartState: Signal<usize>,
    listIdInCard: Signal<usize>,
) -> Element {
    let firstOver = use_signal(cx, || false);
    let over = use_signal(cx, || false);
    let data = ApplicationData::use_app_data(cx);

    render! {
            li {
                match id {
                      0 => render!{
                        div {
                          class: "w-full hover:scale-105 transform transition duration-200",
                          class: if *firstOver.read() {"h-14 bg-slate-600 border-2 border-dashed"} else {"h-8 bg-slate-300"},
                          prevent_default: "ondragover",
                          ondragover: move |_| *firstOver.write() = true,
                          ondragleave: move |_| *firstOver.write() = false,
                          prevent_default: "ondrop",
                          ondrop: move |_| {
                          *firstOver.write() = false;
                          let cc = *dragStartState.read();
                          log::info!("DropToID: {id:?} and dragStartStateID {cc}");
                          // We don't need to change first card
                          //    if card.board_list_id == *list_id {
                          if *dragStartState.read() != 0 || *dragStartState.read() == 0 && *listIdInCard.read() != *list_id {
                          if *listIdInCard.read() == *list_id {
                          let item = signal
                            .read()[*listIdInCard.read()]
                            .write()
                            .remove(*dragStartState.read());
                            signal.read()[*list_id].write().insert(*id, item);
                            } else {
                          let mut item = signal
                             .read()[*listIdInCard.read()]
                             .write()
                             .remove(*dragStartState.read());
                             item.board_list_id = *list_id;
                             signal.read()[*list_id].write().insert(*id, item);
                             }
                             }

                             },
                            "{id}"
                             }

                             },
                             _ => None,
                            },
                div {
                    class: "list h-14 text-rose-100 cursor-move flex flex-row items-center justify-between gap-3 opacity-100 {color}",
                    draggable: "{card.draggable}",
                    onclick: move |event| {
                        log::info!("Clicked! Event: {event:?}");
                    },
                    ondragstart: move |_| {
                        *dragStartState.write() = *id;
                        let cc = *dragStartState.read();
                        let card_board_list_id = card.board_list_id;
                        *listIdInCard.write() = card.board_list_id;
                        log::info!(
                            "DragStartID! {id:?} StateCardID {cc:?} dragstartListINCARD-{card_board_list_id} dragStartLIST-{list_id}"
                        );
                    },
                    svg { class: "fill-current text-white w-7 ml-2", xmlns: card.svgLeft.xmlns, view_box: card.svgLeft.view_box, path { d: card.svgLeft.path.d } }
                    "{card.title} {id}"
                    svg {
                        class: "fill-current text-white w-7 ml-2 cursor-pointer",
                        onclick: move |_| {
                            signal.read()[*list_id].write().remove(*id);
                        },
                        xmlns: card.svgRight.xmlns,
                        view_box: card.svgRight.view_box,
                        path { d: card.svgRight.path.d }
                    }
                }
                div {
                    class: "w-full hover:scale-105 transform transition duration-200",
                    class: if *over.read() {
        "h-14 bg-slate-600 border-2 border-dashed"
    } else {
        "h-8 bg-slate-300"
    },
                    prevent_default: "ondragover",
                    ondragover: move |_| *over.write() = true,
                    ondragleave: move |_| *over.write() = false,
                    prevent_default: "ondrop",
                    ondrop: move |_| {
                        *over.write() = false;
                        let cc = *dragStartState.read();
                        log::info!("DropListID-{list_id} DropID: {id:?} dragStartState {cc}");
                        if *listIdInCard.read() == *list_id {
                            if *dragStartState.read() == *id {} else {
                                if *dragStartState.read() < *id {
                                    let item = signal
                                        .read()[*list_id]
                                        .write()
                                        .remove(*dragStartState.read());
                                    signal.read()[*list_id].write().insert(*id, item);
                                } else {
                                    let item = signal
                                        .read()[*list_id]
                                        .write()
                                        .remove(*dragStartState.read());
                                    signal.read()[*list_id].write().insert(*id + 1, item);
                                }
                            }
                        } else {
                            let mut item = signal
                                .read()[*listIdInCard.read()]
                                .write()
                                .remove(*dragStartState.read());
                            item.board_list_id = *list_id;
                            signal.read()[*list_id].write().insert(*id + 1, item);
                        }
                    },
                    "{id}"
                }
            }
        }
}

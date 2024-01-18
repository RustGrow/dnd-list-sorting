#![allow(non_snake_case)]
use crate::models::app_state::ApplicationData;
use crate::ui::components::list::*;
use crate::ui::components::nav_bar::NavBar;
use dioxus::prelude::*;
use dioxus_signals::*;

#[component]
pub fn DndLists(cx: Scope) -> Element {
    let data = ApplicationData::use_app_data(cx);
    let current_card = use_signal(cx, || 0 as usize);
    let list_id_incard = use_signal(cx, || 0 as usize);

    render! {
        section { class: "flex flex-row",
            NavBar {}
            div { class: "flex flex-col items-center w-full",
                div { class: "text-white h-10", "Hello form lists" }
                div { class: "bg-[#0b0423] flex flex-row gap-2",
                    div {
                        Button {
                            list: data.board.read()[0],
                            list_id: 0,
                            click: InsertAboveCard,
                            text: "Add above"
                        }
                        Divider {}

                        ListBox {
                            box_name: "Box 1",
                            list_id: 0,
                            list: data.board,
                            dragStartState: current_card,
                            listIdInCard: list_id_incard
                        }
                        Divider {}
                        Button {
                            list: data.board.read()[0],
                            list_id: 0,
                            click: InsertBottomCard,
                            text: "Add bottom"
                        }
                    }
                    div {
                        Button {
                            list: data.board.read()[1],
                            list_id: 1,
                            click: InsertAboveCard,
                            text: "Add above"
                        }
                        Divider {}
                        ListBox {
                            box_name: "Box 2",
                            list_id: 1,
                            list: data.board,
                            dragStartState: current_card,
                            listIdInCard: list_id_incard
                        }
                        Divider {}
                        Button {
                            list: data.board.read()[1],
                            list_id: 1,
                            click: InsertBottomCard,
                            text: "Add bottom"
                        }
                    }
                }
            }
        }
    }
}

// https://htmldom.dev/make-a-draggable-element/

use gloo::console::log;
use serde::{Deserialize, Serialize};

use sycamore::prelude::*;
use wasm_bindgen::*;
use web_sys::{DataTransfer, Event};

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(|cx| {
        view! { cx,
            p { "Hello, World!" }
                ContainerWidget()
        }
    });
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
struct ContentItem {
    id: i32,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Contents {
    contents: Vec<ContentItem>,
}

#[component]
fn ContainerWidget<G: Html>(cx: Scope) -> View<G> {
    let contents = create_signal(
        cx,
        vec![
            ContentItem {
                id: 0,
                name: "Test item 0".to_string(),
            },
            ContentItem {
                id: 1,
                name: "Test item 1".to_string(),
            },
            ContentItem {
                id: 2,
                name: "Test item 2".to_string(),
            },
            ContentItem {
                id: 3,
                name: "Test item 3".to_string(),
            },
        ],
    );

    view! { cx,
        div(class = "container") {
            div(class="box") {
                Keyed(
                    iterable=contents,
                    view= move |cx, item| view! { cx, Item(c = item, items = contents) },
                    key=|item| item.id,
                )
            }
        }
    }
}
// modified draggable

#[component(inline_props)]
fn Item<'a, G: Html>(
    cx: Scope<'a>,
    c: ContentItem,
    items: &'a Signal<Vec<ContentItem>>,
) -> View<G> {
    let node_ref = create_node_ref(cx);
    let c_item = create_signal(cx, c);
    let handle_dragstart = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        let drag_event_ref: &web_sys::DragEvent = e.unchecked_ref();
        let drag_event = drag_event_ref.clone();
        let data_transf: DataTransfer = drag_event.data_transfer().unwrap();
        if e.type_().contains("dragstart") {
            data_transf.set_effect_allowed("move");
            data_transf
                .set_data("text/html", &c_item.get().id.to_string())
                .unwrap();

            log!(format!("Transfer {:?}", &c_item.get()));
        }
        dom.set_attribute("style", "opacity: 0.2");
    };

    let handle_dragend = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.set_attribute("style", "opacity: 1");
        log!(format!("{:?}", e.type_()));
    };
    let handle_dragenter = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.add_class("drag-over");
        log!(format!("{:?}", e.type_()));
    };

    let handle_dragover = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        e.prevent_default();
        dom.add_class("drag-over");
    };

    let handle_dragleave = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.remove_class("drag-over");
        log!(format!("{:?}", e));
    };

    let handle_drop = move |e: Event| {
        let drag_event_ref: &web_sys::DragEvent = e.unchecked_ref();
        let drag_event = drag_event_ref.clone();
        let data_transf: DataTransfer = drag_event.data_transfer().unwrap();
        let data = data_transf.get_data("text/html").unwrap();

        log!(format!("{:?}", data.clone()));
        log!(format!("{:?}", &c_item.get()));

        let mut items = items.modify();
        let dragged_index = items
            .iter()
            .position(|i| i.id == data.parse::<i32>().unwrap())
            .unwrap();
        let target_index = items.iter().position(|i| i.id == c_item.get().id).unwrap();
        items.swap(dragged_index, target_index);
    };

    view! { cx,
        div(ref=node_ref, draggable=true, class="item", on:dragstart=handle_dragstart, on:dragend=handle_dragend, on:dragenter=handle_dragenter, on:dragover=handle_dragover, on:dragleave=handle_dragleave, on:drop=handle_drop) {
            (c_item.get().name)
        }
    }
}

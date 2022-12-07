// https://htmldom.dev/make-a-draggable-element/

use gloo::{console::log, utils::document};
use serde::{Deserialize, Serialize};
use sycamore::generic_node::SycamoreElement;
use sycamore::prelude::*;
use wasm_bindgen::*;
use web_sys::{DataTransfer, DragEvent, Element, Event, HtmlElement, MouseEvent};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cat {
    id: &'static str,
    name: &'static str,
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct ItemSwitch {
    contents: RcSignal<Vec<ContentItem>>, //  Vec<ContentItem>,
}

impl ItemSwitch {
    fn switch(self, first: usize, last: usize) -> ItemSwitch {
        self.contents.modify().swap(first, last);
        // self.item.swap(first, last);
        self
    }
    fn values(self) -> Vec<(usize, ContentItem)> {
        let cont = &*self.clone().contents.get();
        cont.into_iter()
            .cloned()
            .enumerate()
            .collect::<Vec<(usize, ContentItem)>>()
    }
}

#[component]
fn ContainerWidget<G: Html>(cx: Scope) -> View<G> {
    let new_items = create_rc_signal(ItemSwitch {
        contents: create_rc_signal(vec![
            ContentItem {
                id: 0,
                name: "Test item 0".to_string(),
            },
            ContentItem {
                id: 0,
                name: "Test item 1".to_string(),
            },
            ContentItem {
                id: 1,
                name: "Test item 2".to_string(),
            },
            ContentItem {
                id: 2,
                name: "Test item 3".to_string(),
            },
        ]),
    });

    provide_context(cx, new_items);

    view! { cx,
        div(class="container") {
             DropZone{}

        }
    }
}

#[component(inline_props)]
fn DraggableItem<G: Html>(cx: Scope, a: usize, c: ContentItem) -> View<G> {
    let node_ref = create_node_ref(cx);
    let a_index = create_signal(cx, a);
    let c_item = create_signal(cx, c);

    let handle_dragstart = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        let drag_event_ref: &web_sys::DragEvent = e.unchecked_ref();
        let drag_event = drag_event_ref.clone();
        let data_transf: DataTransfer = drag_event.data_transfer().unwrap();
        if e.type_().contains("dragstart") {
            data_transf.set_effect_allowed("move");
            data_transf
                .set_data("text/html", &a_index.get().to_string())
                .unwrap();

            log!(format!("Transfer {:?}", &a_index.get()));
        }
        dom.set_attribute("style", "opacity: 0.2");
    };

    let handle_dragend = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.set_attribute("style", "opacity: 1");
    };
    let handle_dragenter = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.add_class("drag-over");
        // log!(format!("{:?}", e.type_()));
    };

    let handle_dragover = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        e.prevent_default();
        dom.add_class("drag-over");
    };

    let handle_dragleave = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.remove_class("drag-over");
    };

    let handle_drop = move |e: Event| {
        let dom = node_ref.get::<DomNode>();

        let drag_event_ref: &web_sys::DragEvent = e.unchecked_ref();
        let drag_event = drag_event_ref.clone();
        let data_transf: DataTransfer = drag_event.data_transfer().unwrap();
        let data = data_transf.get_data("text/html").unwrap();
        log!(format!("{:?}", data.clone()));
        log!(format!("{:?}", &a_index.get()));
        let switch_item = use_context::<RcSignal<ItemSwitch>>(cx);
        let sv = &*switch_item.get();
        sv.clone()
            .switch(data.parse::<usize>().unwrap(), *a_index.get());
    };

    view! { cx,
        div(ref=node_ref, draggable=true, class="item", on:dragstart=handle_dragstart, on:dragend=handle_dragend, on:dragenter=handle_dragenter, on:dragover=handle_dragover, on:dragleave=handle_dragleave, on:drop=handle_drop) {
             //ItemWidget{}
            (c_item.get().name)
        }
    }
}

#[component]
fn DropZone<G: Html>(cx: Scope) -> View<G> {
    let node_ref = create_node_ref(cx);
    let item_contents = use_context::<RcSignal<ItemSwitch>>(cx);
    let values = create_memo(cx, move || {
        let it_sw = &*item_contents.clone().get().contents.get();
        let is = it_sw
            .into_iter()
            .cloned()
            .enumerate()
            .collect::<Vec<(usize, ContentItem)>>();
        is
    });

    view! { cx,
        div(ref=node_ref, class="box") {
             // DraggableItem{}
            Keyed(
                iterable=values,
                    view=|cx, (i, x)|
                        view! { cx,
                                // li { (x) }
                                DraggableItem(a=i, c=x )
                },
                key=|x| x.1.id,
            )
        }
    }
}

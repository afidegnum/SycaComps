// https://htmldom.dev/make-a-draggable-element/

use gloo::console::log;
use serde::{Deserialize, Serialize};

use sycamore::prelude::*;
use wasm_bindgen::*;
use web_sys::{DataTransfer, Event};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub name: String,
}

impl Node {
    fn new(id: i32, parent_id: Option<i32>, name: &str) -> Node {
        Node {
            id,
            parent_id,
            name: name.to_owned(),
        }
    }

    fn has_child(&self, nodes: &[Node]) -> bool {
        nodes.iter().any(|n| n.parent_id == Some(self.id))
    }

    pub fn get_immediate_children<'a>(&'a self, nodes: &'a [Node]) -> Vec<&'a Node> {
        nodes
            .iter()
            .filter(|n| n.parent_id == Some(self.id))
            .collect()
    }
}

#[derive(Debug, Default, Clone)]
pub struct NodeList {
    pub list: Vec<Node>,
}

impl NodeList {
    pub fn get_root_nodes(&self) -> Vec<Node> {
        let mut root_nodes = Vec::new();
        for node in self.list.iter() {
            if node.parent_id.is_none() {
                root_nodes.push(node.clone());
            }
        }
        root_nodes
    }
}

#[derive(Debug, Default, Clone)]
pub struct NodeState {
    pub nodes: RcSignal<NodeList>,
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

#[component(inline_props)]
fn Item<'a, G: Html>(cx: Scope<'a>, n: Node, nodes_sig: &'a Signal<Vec<Node>>) -> View<G> {
    let node_ref = create_node_ref(cx);
    let node_signal = create_signal(cx, n);
    let handle_dragstart = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        let drag_event_ref: &web_sys::DragEvent = e.unchecked_ref();
        let drag_event = drag_event_ref.clone();
        let data_transf: DataTransfer = drag_event.data_transfer().unwrap();
        if e.type_().contains("dragstart") {
            data_transf.set_effect_allowed("move");
            data_transf
                .set_data("text/html", &node_signal.get().id.to_string())
                .unwrap();

            log!(format!("Transfer {:?}", &node_signal.get()));
        }
        //dom.set_attribute("style", "opacity: 0.2");
        dom.add_class("bg-primary bg-opacity-50");
    };

    let handle_dragenter = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        // dom.remove_class("");
        dom.add_class("bg-primary bg-opacity-25");
        log!(format!("{:?}", e.type_()));
    };

    let handle_dragover = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        e.prevent_default();
        dom.remove_class("bg-opacity-25");
        dom.add_class("bg-primary bg-opacity-10");
    };

    let handle_dragleave = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        dom.remove_class("bg-primary");
        log!(format!("{:?}", e));
    };

    let handle_dragend = |e: Event| {
        let dom = node_ref.get::<DomNode>();
        //dom.set_attribute("style", "opacity: 1");
        dom.remove_class("bg-opacity-50");
        dom.add_class("bg-success bg-opacity-10");

        log!(format!("{:?}", e.type_()));
    };
    let handle_drop = move |e: Event| {
        let dom = node_ref.get::<DomNode>();

        let drag_event_ref: &web_sys::DragEvent = e.unchecked_ref();
        let drag_event = drag_event_ref.clone();
        let data_transf: DataTransfer = drag_event.data_transfer().unwrap();
        let data = data_transf.get_data("text/html").unwrap();

        log!(format!("{:?}", data.clone()));
        log!(format!("{:?}", &node_signal.get()));
        dom.remove_class("bg-primary");
        dom.add_class("bg-warning bg-opacity-10");

        let mut items = nodes_sig.modify();
        let dragged_index = items
            .iter()
            .position(|i| i.id == data.parse::<i32>().unwrap())
            .unwrap();
        let target_index = items
            .iter()
            .position(|i| i.id == node_signal.get().id)
            .unwrap();
        items.swap(dragged_index, target_index);
    };

    view! { cx,
        div(ref=node_ref, draggable=true, class="item", on:dragstart=handle_dragstart, on:dragend=handle_dragend, on:dragenter=handle_dragenter, on:dragover=handle_dragover, on:dragleave=handle_dragleave, on:drop=handle_drop) {
            (node_signal.get().name)
        }
    }
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

    let vec_nodes = vec![
        Node::new(1, None, "Node 1"),
        Node::new(2, Some(1), "Node 2"),
        Node::new(3, Some(2), "Node 3"),
        Node::new(4, Some(3), "Node 4"),
        Node::new(5, Some(1), "Node 5"),
        Node::new(6, None, "Node 6"),
        Node::new(7, None, "Node 7"),
        Node::new(8, None, "Node 8"),
        Node::new(9, Some(7), "Node 9"),
        Node::new(10, Some(9), "node 10"),
    ];

    let node_list = create_signal(cx, vec_nodes);

    view! { cx,
        div(class = "container") {
            div(class="box") {
                Keyed(
                    iterable=node_list,
                    view= move |cx, item| view! { cx, Item(n = item, nodes_sig = node_list) },
                    key=|item| item.id,
                )
            }
        }
    }
}
// modified draggable

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

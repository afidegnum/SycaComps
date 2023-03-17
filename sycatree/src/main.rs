use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore::rt::JsCast;
use web_sys::{DataTransfer, Event};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub name: String,
    pub expanded: bool,
}

impl Node {
    fn new(id: i32, parent_id: Option<i32>, name: &str) -> Node {
        Node {
            id,
            parent_id,
            name: name.to_owned(),
            expanded: false,
        }
    }
}

#[component]
fn ContainerWidget<G: Html>(cx: Scope<'_>) -> View<G> {
    let nodes = create_signal(cx, Vec::new());
    create_effect(cx, || {
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
            Node::new(10, Some(9), "Node 10"),
            Node::new(11, Some(9), "Node 11"),
            Node::new(12, Some(7), "node 12"),
        ];

        nodes.set(vec_nodes);
    });

    view! { cx,

             div(class = "container") {
                div(class="d-flex justify-content-center") {
                    div(class="col-3"){
                        Nodes(all_nodes=nodes, parent_id=None, show_children=true)
                    }
                }
            }
    }
}

#[component(inline_props)]
fn Nodes<'cx, G: Html>(
    cx: Scope<'cx>,
    all_nodes: &'cx Signal<Vec<Node>>,
    parent_id: Option<i32>,
    show_children: bool,
) -> View<G> {
    fn Node<'cx, G: Html>(
        cx: Scope<'cx>,
        node: Node,
        all_nodes: &'cx Signal<Vec<Node>>,
    ) -> View<G> {
        let node_ref = create_node_ref(cx);

        let is_dragging = create_signal(cx, false);
        let is_entered = create_signal(cx, false);
        let is_dropped = create_signal(cx, false);

        let handle_dragstart = {
            let node = node.clone();

            move |e: Event| {
                let drag_event_ref: &web_sys::DragEvent = e.unchecked_ref();
                let drag_event = drag_event_ref.clone();
                let data_transfer: DataTransfer = drag_event.data_transfer().unwrap();
                if e.type_().contains("dragstart") {
                    data_transfer.set_effect_allowed("move");
                    data_transfer
                        .set_data("text/plain", &node.id.to_string())
                        .unwrap();
                }
                is_dragging.set(true);
                e.stop_propagation();
            }
        };
        let handle_dragend = |e: Event| {
            is_dragging.set(false);
            e.stop_propagation();
        };

        let handle_dragenter = |e: Event| {
            let drag_event_ref: &web_sys::DragEvent = e.unchecked_ref();
            let drag_event = drag_event_ref.clone();
            let data_transfer: DataTransfer = drag_event.data_transfer().unwrap();
            if e.type_().contains("dragstart") {
                data_transfer.set_effect_allowed("move");
            }

            is_entered.set(true);
            e.prevent_default();
            e.stop_propagation();
        };
        let handle_dragover = |e: Event| {
            e.prevent_default();
        };
        let handle_dragleave = |e: Event| {
            is_entered.set(false);
            e.stop_propagation();
        };

        let target_node_id = node.id;
        let handle_drop = move |e: Event| {
            let drag_event_ref: &web_sys::DragEvent = e.unchecked_ref();
            let drag_event = drag_event_ref.clone();
            let data_transfer: DataTransfer = drag_event.data_transfer().unwrap();
            let data = data_transfer.get_data("text/plain").unwrap();

            let source_node_id: i32 = data.parse().unwrap();

            let mut all_nodes = all_nodes.modify();
            let source_node = all_nodes
                .iter_mut()
                .find(|n| n.id == source_node_id)
                .unwrap();
            source_node.parent_id = Some(target_node_id);

            is_entered.set(false);
            is_dropped.set(true);
            e.stop_propagation();
        };

        let class = create_memo(cx, || {
            let mut class = String::from(" list-group-item");
            if *is_dragging.get() {
                class.push_str(" bg-primary bg-opacity-50");
            }
            if *is_entered.get() && !*is_dragging.get() {
                class.push_str(" bg-primary bg-opacity-25");
            }
            if *is_dropped.get() {
                class.push_str(" bg-success bg-opacity-10");
            }

            class
        });

        let has_children = all_nodes.map(cx, move |nodes| {
            nodes.iter().any(|n| n.parent_id == Some(target_node_id))
        });

        let btn_class = move || {
            format!(
                "px-2 text-primary fa-regular {}",
                if node.expanded {
                    "fa-square-minus"
                } else if *has_children.get() {
                    "fa-square-plus"
                } else {
                    "mx-2"
                }
            )
        };

        let handle_button_click = move |_| {
            let mut all_nodes = all_nodes.modify();
            let node = all_nodes
                .iter_mut()
                .find(|n| n.id == target_node_id)
                .unwrap();

            node.expanded = !node.expanded;
        };

        view! { cx,
            li(
                class=class,
                ref=node_ref,
                draggable=true,
                on:dragstart=handle_dragstart,
                on:dragend=handle_dragend,
                on:dragenter=handle_dragenter,
                on:dragover=handle_dragover,
                on:dragleave=handle_dragleave,
                on:drop=handle_drop,
            ) {
                // button(
                //     class=button_class,
                //     on:click=handle_button_click,
                // ) { (button_text) }
                i(on:click=handle_button_click, class=btn_class())
                span() { (node.name) }
                Nodes(parent_id=Some(node.id), all_nodes = all_nodes, show_children=node.expanded)
            }
        }
    }

    let visible_nodes = if show_children {
        all_nodes.map(cx, move |nodes| {
            nodes
                .iter()
                .cloned()
                .filter(|n| n.parent_id == parent_id)
                .collect()
        })
    } else {
        all_nodes.map(cx, |_| Vec::new())
    };

    view! { cx,
        ul(class="list-group list-group-flush") {
            Indexed(
                iterable=visible_nodes,
                view=move |cx,node| Node(cx,node,all_nodes)
            )
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(|cx| {
        view! { cx,
            ContainerWidget()
        }
    });
}

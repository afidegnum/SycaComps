// use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub name: String,
    pub children: Vec<Node>,
}

impl Node {
    fn new(id: i32, parent_id: Option<i32>, name: &str) -> Node {
        Node {
            id,
            parent_id,
            name: name.to_owned(),
            children: vec![],
        }
    }

    fn has_child(&self, nodes: &[Node]) -> bool {
        nodes.iter().any(|n| n.parent_id == Some(self.id))
    }

    pub fn get_child_node(&self, id: i32) -> Option<Node> {
        for child in self.children.iter() {
            if child.id == id {
                return Some(child.clone());
            }
            if let Some(node) = child.get_child_node(id) {
                return Some(node);
            }
        }
        None
    }
}

#[derive(Debug, Default, Clone)]
pub struct NodeList {
    pub list: Vec<Node>,
}

#[derive(Debug, Default, Clone)]
pub struct NodeState {
    pub nodes: RcSignal<NodeList>,
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

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let vec_nodes = vec![
        Node::new(1, None, "Node 1"),
        Node::new(2, Some(1), "Node 2"),
        Node::new(3, Some(2), "Node 3"),
        Node::new(4, Some(3), "Node 4"),
        Node::new(5, Some(1), "Node 5"),
        Node::new(6, None, "Node 6"),
        Node::new(7, None, "Node 7"),
        Node::new(8, None, "Node 8"),
    ];

    let node_list = NodeList {
        list: vec_nodes.clone(),
    };

    let node_state = NodeState {
        nodes: create_rc_signal(node_list.clone()),
    };
    // let root = build_tree(nodes);

    let node_context = provide_context(cx, node_state);

    let root_nodes = node_context.nodes.get().get_root_nodes();
    // let roots: &[Node] = &root_nodes;
    // let roots: Vec<Node> = root_nodes.to_owned();
    // let y = &roots.clone();
    // let roots = vec_nodes.get_root_nodes();
    let rnodes = create_signal(cx, root_nodes.clone());

    view! { cx,
            div(class="py-4"){
                div(class="container-sm"){
                div(class="row align-items-center"){
                    div(class="col"){
                        div (class="card", style="width: 18rem;") {
                            ul (class="list-group list-group-flush") {
                                Keyed(
                                    iterable=rnodes,
                                    view= move |cx, x|
                                        {
                                            // let child_button = view! { cx, button{ i(class="fa-regular fa-square-plus")}};
                                            // let rnod = rnodes.clone();
                                            // let roots: &[Node] = rnod.get().as_ref().clone().as_slice();
                                            let x_child = x.clone();
                                            let roots = rnodes.get().as_ref().clone();
                                             let n_haschild = format!("--{:#?}", x_child.has_child(&roots));

                                            view! { cx,
                                                    li(class="list-group-item") { (n_haschild) (x.name) hr() div{(format!("{:#?}", x_child))} }

                                    }},
                                    key=|x| x.id,
                                )

                            }
                        }
                    }

                }

            }
            }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    sycamore::render(App);
}

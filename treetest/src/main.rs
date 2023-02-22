use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub name: String,
    // pub children: Vec<Node>,
}

impl Node {
    fn new(id: i32, parent_id: Option<i32>, name: &str) -> Node {
        Node {
            id,
            parent_id,
            name: name.to_owned(),
            // children: vec![],
        }
    }

    fn has_child(&self, nodes: &[Node]) -> bool {
        nodes.iter().any(|n| n.parent_id == Some(self.id))
    }

    pub fn display_immediate_child(&self, nodes: &[Node]) {
        let immediate_child = nodes.iter().find(|n| n.parent_id == Some(self.id));
        match immediate_child {
            Some(child) => println!("{}", child.name),
            None => println!("Node {} has no immediate child.", self.id),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct NodeList {
    pub list: Vec<Node>,
}

// #[derive(Debug, Default, Clone)]
// pub struct NodeState {
//     pub nodes: RcSignal<NodeList>,
// }

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

fn main() {
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
    let node_1 = &vec_nodes[7];
    // node_1.has_child(&vec_nodes);

    println!("{}", node_1.has_child(&vec_nodes));
    println!("{:#?}", node_1);
}

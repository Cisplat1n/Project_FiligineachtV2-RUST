// This module defines the data structures for representing a tree. 
// The tree is represented as a vector of nodes, where each node has a parent and a list of children. 
// The root node is identified by its index in the vector.
pub type NodeId = usize;


// The Tree struct represents the entire tree, containing a vector of nodes and the index of the root node.
#[derive(Debug)]
pub struct Tree {
    pub nodes: Vec<Node>,
    pub root: NodeId,
}

// The Node struct represents a single node in the tree, containing an optional parent node ID, a list of child node IDs, an optional label,
// and an optional branch length.
#[derive(Debug)]
pub struct Node {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub label: Option<String>,
    pub length_to_parent: Option<f64>,
}

// Implementation of the Tree struct, providing a constructor to create a new tree with an empty vector of nodes and a root index of 0.
impl Tree {
    pub fn new() -> Self {
        Tree {
            nodes: Vec::new(),
            root: 0,
        }
    }
}

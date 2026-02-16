// This module defines the data structures for representing a tree. 
// The tree is represented as a vector of nodes, where each node has a parent and a list of children. 
// The root node is identified by its index in the vector.
pub type NodeId = usize;


// The Tree struct represents the entire tree, containing a vector of nodes and the index of the root node.
#[derive(Debug)]
pub struct Tree {
    pub nodes: Vec<Node>, // Vector of nodes in the tree
    pub root: NodeId, // Index of the root node in the nodes vector
}

// The Node struct represents a single node in the tree, containing an optional parent node ID, a list of child node IDs, an optional label,
// and an optional branch length.
#[derive(Debug)]
pub struct Node {
    pub parent: Option<NodeId>, // Optional parent node ID (None for the root node)
    pub children: Vec<NodeId>, // List of child node IDs
    pub label: Option<String>, // Optional label for the node
    pub length_to_parent: Option<f64>, // Optional branch length to the parent node
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

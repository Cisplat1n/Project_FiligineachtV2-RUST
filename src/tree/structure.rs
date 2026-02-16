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


// Fully AI generated code for printing the tree structure in an ASCII format. This method recursively prints each node and its children, using indentation and connectors to visually represent the tree structure. The label and branch length (if available) are also displayed for each node.
impl Tree {
    pub fn print_ascii(&self) {
        self.print_node(self.root, "", true);
    }

    fn print_node(&self, node_id: NodeId, prefix: &str, is_last: bool) {
        let node = &self.nodes[node_id];

        let connector = if prefix.is_empty() {
            ""
        } else if is_last {
            "└── "
        } else {
            "├── "
        };

        let label = node.label.as_deref().unwrap_or("internal");

        let length = match node.length_to_parent {
            Some(l) => format!(" ({})", l),
            None => String::new(),
        };

        println!("{}{}{}{}", prefix, connector, label, length);

        let new_prefix = if prefix.is_empty() {
            String::new()
        } else if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        for (i, &child) in node.children.iter().enumerate() {
            let last = i == node.children.len() - 1;
            self.print_node(child, &new_prefix, last);
        }
    }
}

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
        self.print_node_branches(self.root, "", true);
    }

    pub fn is_leaf(&self, node_id: NodeId) -> bool {
        self.nodes[node_id].children.is_empty()
    }

    pub fn parent(&self, node_id: NodeId) -> Option<NodeId> {
        self.nodes[node_id].parent
    }

    pub fn children(&self, node_id: NodeId) -> &[NodeId] {
        &self.nodes[node_id].children
    }

    pub fn leaves(&self) -> Vec<NodeId> {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_, node)| node.children.is_empty())
            .map(|(i, _)| i)
            .collect()
    }

    pub fn preorder(&self) -> Vec<NodeId> {
        let mut result = Vec::new();
        self.preorder_rec(self.root, &mut result);
        result
    }

    fn preorder_rec(&self, node_id: NodeId, result: &mut Vec<NodeId>) {
        result.push(node_id);

        for &child in &self.nodes[node_id].children {
            self.preorder_rec(child, result);
        }
    }

    pub fn postorder(&self) -> Vec<NodeId> {
    let mut result = Vec::new();
    self.postorder_rec(self.root, &mut result);
    result
    }

    fn postorder_rec(&self, node_id: NodeId, result: &mut Vec<NodeId>) {
        for &child in &self.nodes[node_id].children {
            self.postorder_rec(child, result);
        }

        result.push(node_id);
    }

    pub fn ancestors(&self, node_id: NodeId) -> Vec<NodeId> {
    let mut result = Vec::new();
    let mut current = Some(node_id);

    while let Some(id) = current {
        result.push(id);
        current = self.nodes[id].parent;
    }

    result
    }

    pub fn lca(&self, a: NodeId, b: NodeId) -> Option<NodeId> {
        let ancestors_a = self.ancestors(a);
        let ancestors_b = self.ancestors(b);

        ancestors_a
            .into_iter()
            .find(|id| ancestors_b.contains(id))
    }

    fn print_node_branches(&self, node_id: NodeId, prefix: &str, is_root: bool) {
        let node = &self.nodes[node_id];
        let label = node.label.as_deref().unwrap_or("internal");
        let length = match node.length_to_parent {
            Some(l) => format!(" ({:.3})", l),
            None => String::new(),
        };

        // Print the current node
        if is_root {
            println!("{}{}", label, length);
        } else {
            println!("{}{}{}", prefix, label, length);
        }

        let child_count = node.children.len();
        
        if child_count == 0 {
            return;
        }

        // Calculate spacing for the branches
        let spacing = " ".repeat(label.len() + length.len());
        
        // Print the branch connectors
        if child_count == 1 {
            let child_prefix = format!("{}|", spacing);
            println!("{}", child_prefix);
            self.print_node_branches(node.children[0], &format!("{}  ", spacing), false);
        } else if child_count == 2 {
            // Print branches for two children (the classic look)
            let mid_spacing = " ".repeat(spacing.len());
            let branch_line = format!("{}/{}\\", spacing, mid_spacing);
            println!("{}", branch_line);
            
            // Print left child
            self.print_node_branches(node.children[0], &format!("{}  ", spacing), false);
            
            // Print right child  
            let right_prefix = format!("{}{}", spacing, " ".repeat(spacing.len() + 2));
            self.print_node_branches(node.children[1], &right_prefix, false);
        } else {
            // For more than 2 children, use a simpler format
            for (i, &child) in node.children.iter().enumerate() {
                let connector = if i == child_count - 1 { "\\" } else { "|" };
                let child_prefix = format!("{}{}-- ", spacing, connector);
                println!("{}", child_prefix);
                self.print_node_branches(child, &format!("{}    ", spacing), false);
            }
        }
    }
}
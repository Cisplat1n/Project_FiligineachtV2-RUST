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

    pub fn is_leaf(&self, node_id: NodeId) -> bool { // A node is a leaf if it has no children
        self.nodes[node_id].children.is_empty() // A node is a leaf if it has no children
    }

    pub fn parent(&self, node_id: NodeId) -> Option<NodeId> { // Returns the parent node ID of the given node ID, or None if it is the root node
        self.nodes[node_id].parent // Returns the parent node ID of the given node ID, or None if it is the root node
    }

    pub fn children(&self, node_id: NodeId) -> &[NodeId] { // Returns a slice of child node IDs for the given node ID
        &self.nodes[node_id].children// Returns a slice of child node IDs for the given node ID
    }

    pub fn leaves(&self) -> Vec<NodeId> { // Returns a vector of node IDs that are leaves (nodes with no children)
        self.nodes // Returns a vector of node IDs that are leaves (nodes with no children)
            .iter() // Iterate over the nodes in the tree
            .enumerate() // Enumerate the nodes to get their indices (node IDs)
            .filter(|(_, node)| node.children.is_empty()) // Filter the nodes to keep only those that are leaves (nodes with no children)
            .map(|(i, _)| i) // Map the remaining nodes to their indices (node IDs)
            .collect() // Collect the resulting node IDs into a vector and return it
    }

    pub fn preorder(&self) -> Vec<NodeId> { // Returns a vector of node IDs in preorder traversal (visit the current node before its children)
        let mut result = Vec::new(); // Create a mutable vector to store the result of the preorder traversal
        self.preorder_rec(self.root, &mut result); // Call the recursive helper function to perform the preorder traversal starting from the root node
        result // Return the resulting vector of node IDs in preorder traversal
    }

    fn preorder_rec(&self, node_id: NodeId, result: &mut Vec<NodeId>) { // Recursive helper function for preorder traversal, takes the current node ID and a mutable reference to the result vector
        result.push(node_id); // Add the current node ID to the result vector (visit the current node)

        for &child in &self.nodes[node_id].children { // Iterate over the child node IDs of the current node
            self.preorder_rec(child, result); // Recursively call the helper function for each child node ID, passing the same result vector to accumulate the traversal order
        }
    }

    pub fn postorder(&self) -> Vec<NodeId> { // Returns a vector of node IDs in postorder traversal (visit the children before the current node)
    let mut result = Vec::new(); // Create a mutable vector to store the result of the postorder traversal
    self.postorder_rec(self.root, &mut result); // Call the recursive helper function to perform the postorder traversal starting from the root node
    result // Return the resulting vector of node IDs in postorder traversal
    }

    fn postorder_rec(&self, node_id: NodeId, result: &mut Vec<NodeId>) { // Recursive helper function for postorder traversal, takes the current node ID and a mutable reference to the result vector
        for &child in &self.nodes[node_id].children { // Iterate over the child node IDs of the current node
            self.postorder_rec(child, result); // Recursively call the helper function for each child node ID, passing the same result vector to accumulate the traversal order
        }

        result.push(node_id); // Add the current node ID to the result vector after visiting all its children (visit the current node)
    }

    pub fn ancestors(&self, node_id: NodeId) -> Vec<NodeId> { // Returns a vector of node IDs that are ancestors of the given node ID, starting from the given node and going up to the root
    let mut result = Vec::new(); // Create a mutable vector to store the result of the ancestors traversal
    let mut current = Some(node_id); // Start with the given node ID as the current node

    while let Some(id) = current { // Loop while there is a current node ID (not None)
        result.push(id); // Add the current node ID to the result vector (visit the current node)
        current = self.nodes[id].parent; // Update the current node ID to its parent node ID for the next iteration (move up to the parent node)
    }

    result // Return the resulting vector of node IDs that are ancestors of the given node ID, starting from the given node and going up to the root
    }

    pub fn lca(&self, a: NodeId, b: NodeId) -> Option<NodeId> { // Returns the node ID of the lowest common ancestor (LCA) of the two given node IDs, or None if there is no common ancestor (which should not happen in a well-formed tree)
        let ancestors_a = self.ancestors(a); // Get the ancestors of node a, which includes a itself and all its ancestors up to the root
        let ancestors_b = self.ancestors(b);// Get the ancestors of node b, which includes b itself and all its ancestors up to the root

        ancestors_a // Iterate over the ancestors of node a
            .into_iter()// Convert the ancestors of node a into an iterator
            .find(|id| ancestors_b.contains(id)) // Find the first ancestor of node a that is also an ancestor of node b (the lowest common ancestor)
    }

    pub fn node_label(&self, node_id: NodeId) -> Option<&str> { // Returns the label of the node with the given node ID, or None if the node has no label
    self.nodes[node_id].label.as_deref() // Return the label of the node with the given node ID as an Option<&str>, converting from Option<String> using as_deref()
    }

    pub fn taxa(&self) -> Vec<&str> { // Returns a vector of labels for the leaf nodes (taxa) in the tree, filtering out any nodes that do not have labels
    self.leaves() // Get the node IDs of the leaf nodes in the tree
        .into_iter() // Convert the leaf node IDs into an iterator
        .filter_map(|id| self.nodes[id].label.as_deref()) // For each leaf node ID, get its label as an Option<&str> and filter out any nodes that do not have labels (None values)
        .collect() // Collect the resulting labels of the leaf nodes into a vector and return it
    }

    pub fn depth(&self, node_id: NodeId) -> usize { // Returns the depth of the node with the given node ID, defined as the number of edges from the node to the root (the root has depth 0)
    let mut depth = 0; // Initialise the depth counter to 0
    let mut current = self.parent(node_id); // Start with the parent of the given node ID as the current node (the root will have None as its parent)

    while let Some(p) = current { // Loop while there is a current node ID (not None)
        depth += 1; // Increment the depth counter for each edge we move up to the parent node
        current = self.parent(p); // Update the current node ID to its parent node ID for the next iteration (move up to the parent node)
    }

    depth // Return the calculated depth of the node with the given node ID, which is the number of edges from the node to the root
    }   

    // AI generated ASCII tree printing method
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
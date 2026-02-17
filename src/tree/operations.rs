// This file contains additional operations on the tree structure
use super::structure::{Tree, NodeId};




// Patristic distance and Topological distance methods for the Tree structure
impl Tree { // Calculate the topological distance (number of edges) between two nodes
    pub fn topological_distance(&self, a: NodeId, b: NodeId) -> Option<usize> { // Find the lowest common ancestor (LCA) of the two nodes
        let lca = self.lca(a, b)?; // Calculate the depth of each node and the LCA
        let da = self.depth(a); // Depth of node a
        let db = self.depth(b); // Depth of node b
        let dl = self.depth(lca); // Depth of the LCA

        Some(da + db - 2 * dl) // The topological distance is the sum of the depths minus twice the depth of the LCA
    }

    pub fn patristic_distance(&self, mut a: NodeId, mut b: NodeId) -> Option<f64> { // Calculate the patristic distance (sum of branch lengths) between two nodes
        let lca = self.lca(a, b)?; // Initialise the distance to zero
        let mut dist = 0.0; // Traverse from node a to the LCA, adding the branch lengths

        while a != lca { // Add the length to the parent node, if it exists
            dist += self.nodes[a].length_to_parent.unwrap_or(0.0); // Move up to the parent node
            a = self.parent(a)?; // Traverse from node b to the LCA, adding the branch lengths
        } 

        while b != lca { // Add the length to the parent node, if it exists
            dist += self.nodes[b].length_to_parent.unwrap_or(0.0); // Move up to the parent node
            b = self.parent(b)?; // Return the total patristic distance
        } 

        Some(dist) // The patristic distance is the total length of the path from a to b through the LCA
    }


    /// Returns a Vec where index = NodeId and value = number of descendant leaves
    pub fn compute_subtree_leaf_counts(&self) -> Vec<usize> { // Initialise a vector to hold the leaf counts for each node
        let mut counts = vec![0; self.nodes.len()]; // Traverse the tree in postorder to compute leaf counts

        // Postorder traversal ensures children computed before parent
        for node in self.postorder() { // If the node is a leaf, it contributes 1 to the count
            if self.is_leaf(node) { // A leaf node has a count of 1
                counts[node] = 1; // If the node is not a leaf, its count is the sum of the counts of its children
            } else { // 
                let mut sum = 0; // Sum the counts of all child nodes
                for &child in &self.nodes[node].children { // Add the count of each child to the sum
                    sum += counts[child]; // The count for the current node is the total number of descendant leaves
                counts[node] = sum; // This method allows us to quickly determine how many leaves are in the subtree rooted at any given node
            }
        }

        counts// Return the vector of leaf counts for each node
    }
}









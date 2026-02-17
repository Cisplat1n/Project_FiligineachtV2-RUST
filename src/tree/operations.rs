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
}









// This module is responsible for parsing tree data from various formats (e.g., Newick, Nexus).

use super::structure::{Tree, Node, NodeId}; // Importing the Tree, Node, and NodeId types from the structure module.

// enums for supported formats and error handling
#[derive(Debug)]
pub enum ParseError {
    UnexpectedEnd,
    UnexpectedToken(char),
    UnbalancedParentheses,
    InvalidBranchLength,
}




// The parse_newick function takes a Newick formatted string and returns a Tree structure.
pub fn parse_nwk(input: &str) -> Result<Tree, ParseError> { // 
    let mut nodes: Vec<Node> = Vec::new(); // Vector to hold the nodes of the tree as they are parsed.
    let mut stack: Vec<NodeId> = Vec::new(); // Stack to keep track of the current parent nodes as we parse through the tree structure.

    let mut chars = input.chars().peekable(); // Create a peekable iterator over the characters of the input string
    let mut root: Option<NodeId> = None; // Variable to hold the index of the root node once it is identified.

    while let Some(c) = chars.next() { // Loop through each character in the input string
        match c { // Match the character to determine how to parse it
            '(' => { // When we encounter an opening parenthesis, we create a new node and push it onto the stack.
                let id = nodes.len(); // The ID of the new node is the current length of the nodes vector, which will be its index.

                nodes.push(Node { // Create a new node with the appropriate parent and push it onto the nodes vector.
                    parent: stack.last().copied(), // The parent of the new node is the last element on the stack (the current parent node), or None if the stack is empty.
                    children: Vec::new(), // Initialise the children vector for the new node.
                    label: None, // The label for this node is None, as it is an internal node created by the '(' character.
                    length_to_parent: None, // The branch length to the parent is None for now, as it may be specified later in the Newick format after the label. We will update this if we encounter a colon followed by a branch length value.
                });

                if let Some(&parent_id) = stack.last() { // If there is a parent node on the stack, we add the new node as a child of that parent.
                    nodes[parent_id].children.push(id); // Add the new node's ID to the children of the parent node.
                }

                stack.push(id); // Push the new node's ID onto the stack, making it the current parent for any subsequent nodes until we encounter a closing parenthesis.

                if root.is_none() { // If the root node has not been set yet, we set it to the ID of the new node, as this is the first node we encounter and will be the root of the tree.
                    root = Some(id); // Set the root node ID to the current node ID if it hasn't been set yet.
                } // Note: The root node is the first node created when we encounter the first '(' character, and it will be the parent of all other nodes in the tree.
            }

            ')' => {
                let node_id = stack.pop().ok_or(ParseError::UnbalancedParentheses)?;

                // check for branch length
                if let Some(&':') = chars.peek() {
                    chars.next(); // consume ':'

                    let mut number = String::new();

                    while let Some(&next) = chars.peek() {
                        if next == ',' || next == ')' || next == ';' {
                            break;
                        }
                        number.push(chars.next().unwrap());
                    }

                    let length = number
                        .parse::<f64>()
                        .map_err(|_| ParseError::InvalidBranchLength)?;

                    nodes[node_id].length_to_parent = Some(length);
                }
            }

            ',' => continue, // A comma indicates a sibling node, so we simply continue to the next character without doing anything special.

            ';' => break, // A semicolon indicates the end of the tree definition, so we break out of the loop.

            c if c.is_whitespace() => continue, // If the character is whitespace, we ignore it and continue to the next character.

            _ => { // For any other character, we assume it is the start of a label for a node. We read the label until we encounter a comma, closing parenthesis, or semicolon.
                let mut label = String::new(); // Create a new string to hold the label of the node.
                label.push(c); // Push the first character of the label onto the string.

                while let Some(&next) = chars.peek() { // Peek at the next character to see if it is part of the label. If it is a comma, closing parenthesis, or semicolon, we stop reading the label.
                    if next == ',' || next == ')' || next == ';'|| next == ':'{ // If the next character is a comma, closing parenthesis, semicolon, or colon (indicating a branch length), we break out of the loop, as these characters indicate the end of the label.
                        break; // Break out of the loop if we encounter a character that indicates the end of the label.
                    } // If the next character is not a comma, closing parenthesis, semicolon, or colon, we read it as part of the label and continue.
                    label.push(chars.next().unwrap()); // Push the next character onto the label string and consume it from the iterator.
                } // After reading the label, we create a new node with the label and add it to the nodes vector. We also update the parent-child relationships based on the current state of the stack.
                
                let mut length_to_parent = None; // Variable to hold the branch length to the parent node, if specified in the Newick format.

                if let Some(&':') = chars.peek() { // If the next character is a colon, it indicates that there is a branch length specified for this node. We read the branch length and store it in the length_to_parent variable.
                    chars.next(); // consume ':'

                    let mut number = String::new(); // Create a new string to hold the characters of the branch length as we read them from the input.

                    while let Some(&next) = chars.peek() { // Peek at the next character to see if it is part of the branch length. If it is a comma, closing parenthesis, or semicolon, we stop reading the branch length.
                        if next == ',' || next == ')' || next == ';' { // If the next character is a comma, closing parenthesis, or semicolon, we break out of the loop, as these characters indicate the end of the branch length.
                            break; // Break out of the loop if we encounter a character that indicates the end of the branch length.
                        } // If the next character is not a comma, closing parenthesis, or semicolon, we read it as part of the branch length and continue.
                        number.push(chars.next().unwrap()); // Push the next character onto the number string and consume it from the iterator.
                    } // After reading the branch length as a string, we attempt to parse it as a floating-point number and store it in the length_to_parent variable. If parsing fails, we return an error indicating that the branch length is invalid.

                    length_to_parent = Some( // Attempt to parse the branch length string as a floating-point number. If parsing fails, we return an error indicating that the branch length is invalid.
                        number.parse::<f64>() // Parse the number string as a floating-point number. If parsing fails, it will return an error, which we map to our custom ParseError::InvalidBranchLength error type.
                            .map_err(|_| ParseError::InvalidBranchLength)? // If parsing is successful, store it in the length_to_parent variable.
                    );
                }

                let id = nodes.len(); // The ID of the new node is the current length of the nodes vector, which will be its index.

                nodes.push(Node { // Create a new node with the appropriate parent and label, and push it onto the nodes vector.
                    parent: stack.last().copied(), // The parent of the new node is the last element on the stack (the current parent node), or None if the stack is empty.
                    children: Vec::new(), // Initialise the children vector for the new node, as it may have children if it is an internal node.
                    label: Some(label), // Set the label of the new node to the string we just read from the input.
                    length_to_parent, // <-- match struct
                });

                if let Some(&parent_id) = stack.last() { // If there is a parent node on the stack, we add the new node as a child of that parent.
                    nodes[parent_id].children.push(id); // Add the new node's ID to the children of the parent node, establishing the parent-child relationship in the tree structure.
                } else { // If there is no parent node on the stack, it means this node is not properly nested within the tree structure, and we return an error.
                    return Err(ParseError::UnexpectedToken(c)); // Return an error if we encounter a label that is not properly nested within the tree structure (i.e., it is not a child of any node).
                }
            }
        }
    }

    if !stack.is_empty() { // After processing all characters, if the stack is not empty, it means we have unbalanced parentheses (i.e., there are more opening parentheses than closing parentheses), and we return an error.
        return Err(ParseError::UnbalancedParentheses); // Return an error if we have unbalanced parentheses, which indicates that the tree structure is not properly defined in the input string.
    }

    let root_id = root.ok_or(ParseError::UnexpectedEnd)?; // If we never set the root node (i.e., we never encountered an opening parenthesis), we return an error indicating that the input ended unexpectedly without defining a valid tree structure.

    Ok(Tree { nodes, root: root_id }) // If we successfully parsed the input string and constructed the tree structure, we return the Tree object containing the vector of nodes and the index of the root node.
}


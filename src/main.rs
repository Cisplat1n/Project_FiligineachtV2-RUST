use filigineacht_rs::tree::parser::parse_nwk;




fn main() {
    match parse_nwk("(A:0.1,(B:0.2,C:0.3)X:0.4)Root;") {
        Ok(tree) => {
            println!("Leaves: {:?}", tree.leaves());
            println!("Preorder: {:?}", tree.preorder());
            println!("Postorder: {:?}", tree.postorder());
            for id in tree.preorder() {
                println!("{:?}", tree.node_label(id));
                }
        }
        Err(e) => println!("Parse error: {:?}", e),
    }
}


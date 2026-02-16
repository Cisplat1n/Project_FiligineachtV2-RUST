use filigineacht_rs::tree::parser::parse_nwk;




fn main() {
    match parse_nwk("(A:0.1,(B:0.2,C:0.3)X:0.4)Root;") {
        Ok(tree) => tree.print_ascii(),
        Err(e) => println!("Parse error: {:?}", e),
    }
}

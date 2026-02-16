use filigineacht_rs::tree::parser::parse_nwk;




fn main() {
    println!("{:#?}", parse_nwk("(A,B);"));
    println!("{:#?}", parse_nwk("(A,(B,C));"));
    println!("{:#?}", parse_nwk("(A,B"));
    println!("{:#?}", parse_nwk("(A:0.1,(B:0.2,C:0.3):0.4);"));
    let tree = parse_nwk("(A:0.1,(B:0.2,C:0.3):0.4);").unwrap();
    tree.print_ascii();
}


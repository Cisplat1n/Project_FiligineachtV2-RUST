use filigineacht_rs::tree::parser::parse_nwk;




fn main() {
    println!("{:#?}", parse_nwk("(A,B);"));
    println!("{:#?}", parse_nwk("(A,(B,C));"));
    println!("{:#?}", parse_nwk("(A,B"));
}

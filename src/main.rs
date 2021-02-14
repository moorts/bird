mod arith;

pub use crate::arith::parser::{tokenize, to_expression_tree};

fn main() {
    println!("{:?}", to_expression_tree(tokenize(String::from("3 + 4 * (4 + 2)"))));
}

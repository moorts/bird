mod arith;

pub use crate::arith::parser::{to_expression_tree, tokenize};

fn main() {
    println!(
        "{:?}",
        to_expression_tree(tokenize(String::from("3 + 4 * (4 + 2)")))
    );
}

use crate::lexer::ordered_keywords;

pub mod error;
pub mod lexer;

fn main() {
    println!("Hello, world!");
    for k in ordered_keywords() {
        println!("{:?}[{}]",k, k)
    }
}

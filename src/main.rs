use std::{fs::File, io::Result};

use lexer::{CodeSource, FileCodeSourceImpl};
pub mod ast;
pub mod error;
mod helper;
pub mod lexer;
mod simple_tree;
mod tree_old;
pub mod tree;
mod weakbox;
fn main() -> Result<()> {
    //Decide post or prefixed types.
    let cs = FileCodeSourceImpl::new(File::open("./Testfile.ch")?);
    for token in cs.iter()? {
        println!("{}", token?)
    }
    Ok(())
}

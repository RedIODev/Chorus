use std::{fs::File, io::Result};

use lexer::{CodeSource, FileCodeSourceImpl};
pub mod error;
mod helper;
pub mod lexer;
pub mod ast;
mod tree;

fn main() -> Result<()> {
    //Decide post or prefixed types.
    let cs = FileCodeSourceImpl::new(File::open("./Testfile.ch")?);
    for token in cs.iter()? {
        println!("{}", token?)
    }
    Ok(())
}

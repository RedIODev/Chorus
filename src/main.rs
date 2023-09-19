use std::{io::Result, fs::File};

use lexer::{FileCodeSourceImpl, CodeSource};


pub mod error;
pub mod lexer;
mod helper;

fn main() -> Result<()> {
    //rethink error handling 
    let cs = FileCodeSourceImpl::new(File::open("./Testfile.ch")?);
    for token in cs.iter()? {
        let token = token?;
        println!("[{}]@{:?}", token.raw(), token.source_position())
    }
   Ok(()) 
}

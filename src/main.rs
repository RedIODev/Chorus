use std::{error::Error, fs::File};

mod tokenizer;

fn main() -> TmpResult<()> {
    //Decide post or prefixed types.
    let file = File::open("./Testfile.ch")?;
    
        Ok(())
}

pub type TmpResult<T> = Result<T, Box<dyn Error>>;
use std::{env::args, error::Error};

use crate::tokenizer::{Tokenizer};

mod tokenizer;
mod parser;


#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> TmpResult<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    let file = args().nth(1).expect("Provide a source file!");
    let mut token_len = 0;
    let mut token_count = 0;
    println!("'{file}'");
    let tokenizer = Tokenizer::new(&file)?;
            for token in tokenizer {
                token_len += token.token.line_len();
                token_count+=1;
                if token_count < 1000 {
                    println!("Token:[{:?}]", token)
                }
            }
println!("Token_len:{token_len}, Token_count:{token_count}");
        Ok(())
}

pub type TmpResult<T> = Result<T, Box<dyn Error>>;
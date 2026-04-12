use std::error::Error;

use crate::tokenizer::{Tokenizer};

mod tokenizer;


#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> TmpResult<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let mut token_len = 0;
    let mut token_count = 0;
    let tokenizer = Tokenizer::new("./Testfile.ch")?;
            for token in tokenizer {
                token_len += token.token.line_len();
                token_count+=1;
                if token_count < 4000 {
                    println!("Token:[{:?}]", token)
                }
            }
println!("Token_len:{token_len}, Token_count:{token_count}");
        Ok(())
}

pub type TmpResult<T> = Result<T, Box<dyn Error>>;
use std::{error::Error, fs::File};

use crate::tokenizer::{Keyword, Tokenizer};

mod tokenizer;

fn main() -> TmpResult<()> {

    let mut token_len = 0;
    let mut token_count = 0;
    for _ in 0..10_000 {
        let tokenizer = Tokenizer::new("./Testfile.ch")?;
        for token in tokenizer {
            token_len += token.token.line_len();
            token_count+=1;
        }
    }
    println!("Token_len:{token_len}, Token_count:{token_count}");
    // let tokenizer = Tokenizer::new("./Testfile.ch")?;
    // for token in tokenizer {
    //     println!("Token:[{:?}]", token)
    // }

    // println!();
    // let tokenizer = Tokenizer::new("./Testfile.ch")?;
    // let mut ll = 0;
    // for token in tokenizer {
    //     use tokenizer::Token;
    //     if ll != token.line {
    //         println!();
    //     }
    //     match token.token {
    //         Token::Comment(c) => print!("{} ", c.comment()),
    //         Token::Identifier(i) => print!("{i} "),
    //         Token::Keyword(k) => print!("{} ", <&str>::from(k)),
    //         Token::Symbol(s) => print!("{:?} ", s as u8 as char)
    //     }
    //     ll = token.line;
    // }
    // println!();
        Ok(())
}

pub type TmpResult<T> = Result<T, Box<dyn Error>>;
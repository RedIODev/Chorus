use std::{error::Error, fs::File};

use crate::tokenizer::{Keyword, Tokenizer};

mod tokenizer;

fn main() -> TmpResult<()> {
    //Decide post or prefixed types.
    //let string: &str = Keyword::Break.into();
    //println!("{string}");
    let tokenizer = Tokenizer::new("./Testfile.ch")?;
    for token in tokenizer {
        println!("Token:[{:?}]", token)
    }

    println!();
    let tokenizer = Tokenizer::new("./Testfile.ch")?;
    let mut ll = 0;
    for token in tokenizer {
        use tokenizer::Token;
        if ll != token.line {
            println!();
        }
        match token.token {
            Token::Comment(c) => print!("{} ", c.comment()),
            Token::Identifier(i) => print!("{i} "),
            Token::Keyword(k) => print!("{} ", <&str>::from(k)),
            Token::Symbol(s) => print!("{:?} ", s as u8 as char)
        }
        ll = token.line;
    }
    //let file = File::open("./Testfile.ch")?;
    
        Ok(())
}

pub type TmpResult<T> = Result<T, Box<dyn Error>>;
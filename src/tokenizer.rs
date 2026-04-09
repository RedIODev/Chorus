use std::iter::Peekable;
use std::{fs::File, path::Path};
use std::io::{self, BufRead};

use crate::TmpResult;

pub struct Tokenizer {
    lines: ReaderLines,
    line: u32,
    column: u32
}

impl Tokenizer {
    pub fn new(file: impl AsRef<Path>) -> TmpResult<Tokenizer> {
        Ok(Tokenizer { lines: read_lines(file)?, line: 0, column: 0 })
    }

    pub fn peek(&self) -> Option<PosToken> {
        let line = self.lines.peek()?;
    }

    pub fn next(&mut self) -> Option<PosToken> {
        let token = self.peek();
        let Some(token) = token else {
            return None;
        };

    }
}

type ReaderLines = Peekable<io::Lines<io::BufReader<File>>>;

fn read_lines(filename: impl AsRef<Path>) -> io::Result<ReaderLines> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().peekable())
}

pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Symbol(Symbol)
}

pub enum Keyword {
    Var,
    Fn,
    For,
    While,
    Loop,
    If,
    Else,
    Union,
    Enum,
    Type,
    Struct,
    Return,
    Yield,
    Continue,
    Break,
    Switch,
    This,
    Interface,
    Namespace,
    Export,
    Import,
    Macro,
}

pub enum Symbol {
    BraceOpen,
    BraceClosed,
    CurlyBraceOpen,
    CurlyBraceClosed,
    SquareBraceOpen,
    SquareBraceClosed,
    And,
    Pipe,
    Slash,
    Equals,
    Comma,
    SemiColon,
    Colon,
    Period,
    Minus,
    Plus,
    Asterisk,
    QuestionMark,
    ExclamationMark,
    DoubleQuote,
    Quote,
    Tilde,
    Hashtag,
    GreaterThen,
    SmallerThen,
    Hat,

}

pub struct PosToken {
    pub token: Token,
    pub line: u32,
    pub column: u32,
    pub length: u32,
}
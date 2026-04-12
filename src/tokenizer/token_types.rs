#![allow(dead_code)]

use num_enum::TryFromPrimitive;
use strum_macros::{IntoStaticStr, VariantArray};

#[derive(Debug)]
pub struct PosToken {
    pub token: Token,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Symbol(Symbol),
    Comment(Comment),
}

impl Token {
    pub fn line_len(&self) -> u32 {
        match &self {
            Token::Comment(Comment::Line(_)) => 0,
            Token::Comment(Comment::Doc(..)) => 0,
            Token::Comment(Comment::Block { columns, .. }) => *columns,
            Token::Identifier(identifier) => identifier.len() as u32,
            Token::Keyword(keyword) => <&str>::from(keyword).len() as u32,
            Token::Symbol(_) => 1,
        }
    }

    pub fn new_lines(&self) -> u32 {
        match &self {
            Token::Comment(Comment::Line(_)) => 1,
            Token::Comment(Comment::Block { new_line_count, .. }) => *new_line_count,
            _ => 0,
        }
    }
}

#[derive(Debug)]
pub enum Comment {
    Line(String),
    Doc(String, u32),
    Block {
        comment: String,
        new_line_count: u32,
        columns: u32,
    },
}

impl Comment {
    pub fn comment(&self) -> &str {
        match self {
            Comment::Block { comment, .. } => comment,
            Comment::Doc(comment, _) => comment,
            Comment::Line(comment) => comment,
        }
    }
}

#[derive(IntoStaticStr, VariantArray, Clone, Copy, Debug)]
#[strum(serialize_all = "lowercase")]
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
    Implement,
    Where,
    Const,
    Namespace,
    Export,
    Import,
    Macro,
}

#[derive(TryFromPrimitive, Clone, Copy, Debug)]
#[repr(u8)]
#[non_exhaustive]
pub enum Symbol {
    BraceOpen = b'(',
    BraceClosed = b')',
    CurlyBraceOpen = b'{',
    CurlyBraceClosed = b'}',
    SquareBraceOpen = b'[',
    SquareBraceClosed = b']',
    And = b'&',
    Pipe = b'|',
    Slash = b'/',
    Equals = b'=',
    Comma = b',',
    SemiColon = b';',
    Colon = b':',
    Period = b'.',
    Minus = b'-',
    Plus = b'+',
    Asterisk = b'*',
    QuestionMark = b'?',
    ExclamationMark = b'!',
    DoubleQuote = b'"',
    Quote = b'\'',
    Tilde = b'~',
    Hashtag = b'#',
    GreaterThen = b'>',
    SmallerThen = b'<',
    Hat = b'^',
    Degree = b'\xB0', // "°" ASCII character
    Backslash = b'\\',
    Percent = b'%',
    Dollar = b'$',
    At = b'@',
}
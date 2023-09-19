pub mod impls;
mod tokenizer;

use std::fs::File;

use strum_macros::{EnumCount, EnumIter};

pub trait Token {
    fn raw(&self) -> &str;
    fn source_position(&self) -> SourcePosition;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SourcePosition {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, Copy, EnumIter, EnumCount, PartialEq, Eq, PartialOrd, Ord)]
pub enum Keyword {
    //single character keywords
    BracketRoundOpen,
    BracketRoundClose,
    BracketCurlyOpen,
    BracketCurlyClose,
    BracketSquareOpen,
    BracketSquareClose,
    BracketDiamondOpen,
    BracketDiamondClose,
    QuoteSingle,
    QuoteDouble,
    MarkExclamation,
    MarkQuestion,
    Pipe,
    SlashForward,
    SlashBackward,
    Percent,
    And,
    Equals,
    Star,
    Plus,
    Minus,
    Underscore,
    Comma,
    Colon,
    ColonSemi,
    Period,
    Tilde,
    Hashtag,
    Caret,
    Degree,
    At,

    //word keywords
    Namespace,
    Import,
    Public,
    Local,
    Const,
    Mut,
    Struct,
    Var,
    Unsafe,
    Extern,
    Inline,
    Interface,
    Type,
    As,
    Fn,
    Break,
    Continue,
    If,
    Else,
    Switch,
    For,
    While,
    Return,
    Enum,
    Union,
    Static,
    Staged,
    Yield,
    In,
    Unsigned,
    With,
    Where,
    Super,
    Satisfies,
    Is,
    Alloc,
    Implement,
    Ref,
    _Self,
    //lexer macro keywords
    __Line,
    __File,
}

pub struct KeywordToken {
    keyword: Keyword,
    source_position: SourcePosition,
}

pub struct IdentifierToken {
    raw: Box<str>,
    source_position: SourcePosition,
}

pub type LineIter<E> = Box<dyn Iterator<Item = Result<String, E>>>;

pub trait CodeSource {
    type Iter: Iterator<Item = Result<String, Self::Error>>;
    type Error;
    fn desc(&self) -> &str;

    fn iter(&self) -> Result<TokenStream<Self>, Self::Error> where Self: Sized;
}

pub trait FileCodeSource: CodeSource {
    fn filename(&self) -> &str;
}

pub struct FileCodeSourceImpl {
    file:File
}

pub struct TokenStream<CS> 
where CS: CodeSource,
{
    iter:PausableIter<CS::Iter>,
    current_pos: SourcePosition
}

pub enum TokenResult<E> {
    None,
    Some(Box<dyn Token>),
    Err(E)
}

use crate::helper::PausableIter;
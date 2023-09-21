use std::fs::File;

use strum_macros::{EnumCount, EnumIter, EnumVariantNames};

pub trait TokenTrait {
    fn raw(&self) -> &str;
    fn source_position(&self) -> SourcePosition;
}

#[derive(Debug, EnumVariantNames)]
#[repr(usize)]
pub enum Token {
    Keyword(KeywordToken),
    Identifier(IdentifierToken),
    Comment(CommentToken),
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
    Export,
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
    Not,
    Alloc,
    Implement,
    Ref,
    _Self,
    //lexer macro keywords
    __Line,
    __File,
}

#[derive(Debug)]
pub enum CommentType {
    LineComment,
    DocComment,
    BlockComment,
}

#[derive(Debug)]
pub struct KeywordToken {
    pub(super) keyword: Keyword,
    pub(super) source_position: SourcePosition,
}

#[derive(Debug)]
pub struct IdentifierToken {
    pub(super) raw: Box<str>,
    pub(super) source_position: SourcePosition,
}

#[derive(Debug)]
pub struct CommentToken {
    pub(super) comment_type: CommentType,
    pub(super) raw: Box<str>,
    pub(super) source_position: SourcePosition,
}

pub trait CodeSource {
    type Iter: Iterator<Item = Result<String, Self::Error>>;
    type Error;
    fn desc(&self) -> &str;

    fn iter(&self) -> Result<TokenStream<Self>, Self::Error>
    where
        Self: Sized;
}

pub trait FileCodeSource: CodeSource {
    fn filename(&self) -> &str;
}

pub struct FileCodeSourceImpl {
    pub(super) file: File,
}

pub struct TokenStream<CS>
where
    CS: CodeSource,
{
    pub(super) iter: PausableIter<CS::Iter>,
    pub(super) current_pos: SourcePosition,
}

pub enum TokenResult<E> {
    None,
    Some(Token),
    Err(E),
}

use crate::helper::PausableIter;

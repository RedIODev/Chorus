pub mod impls;
mod tokenizer;
use strum_macros::{EnumCount, EnumIter};

pub trait Token {
    fn raw(&self) -> &str;
    fn source_position(&self) -> SourcePosition;
}

#[derive(Clone, Copy)]
pub struct SourcePosition {
    pub row: u32,
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
    //double character keywords
    CommentLine,
    CommentStart,
    CommentEnd,
    EqualsDouble,
    EqualsLess,
    EqualsMore,
    EqualsNot,
    EqualsStar,
    EqualsSlash,
    EqualsPlus,
    EqualsMinus,
    EqualsPercent,
    EqualsAnd,
    EqualsPipe,
    EqualsTide,
    EqualsCaret,
    EqualsMarkQuestion,
    EqualsHashtag,
    EqualsDegree,
    EqualsAt,
    ShiftLeft,
    ShiftRight,
    AndDouble,
    PipeDouble,
    Arrow,
    ArrowFat,
    ColonDoubleDouble,
    //triple character keywords
    EqualsShiftLeft,
    EqualsShiftRight,
    EqualsAndDouble,
    EqualsPipeDouble,
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

pub use tokenizer::tokenize;

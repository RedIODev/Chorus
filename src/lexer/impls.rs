use std::fmt::Display;

use strum::IntoEnumIterator;

use crate::error::LexerError;

use super::{IdentifierToken, Keyword, KeywordToken, SourcePosition, Token};

impl SourcePosition {
    pub fn new(row: u32, column: u32) -> Self {
        Self { row, column }
    }
}

impl From<(u32, u32)> for SourcePosition {
    fn from(value: (u32, u32)) -> Self {
        SourcePosition::new(value.0, value.1)
    }
}

impl TryFrom<&str> for Keyword {
    type Error = LexerError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        for keyword in Keyword::iter() {
            if keyword.raw() == value {
                return Ok(keyword);
            }
        }
        Err(LexerError::KeywordNotFound(value.to_owned()))
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw())
    }
}

impl Keyword {
    pub const fn raw(&self) -> &str {
        use Keyword as K;
        match self {
            K::BracketRoundOpen => "(",
            K::BracketRoundClose => ")",
            K::BracketCurlyOpen => "{",
            K::BracketCurlyClose => "}",
            K::BracketSquareOpen => "[",
            K::BracketSquareClose => "]",
            K::BracketDiamondOpen => "<",
            K::BracketDiamondClose => ">",
            K::QuoteSingle => "'",
            K::QuoteDouble => "\"",
            K::MarkExclamation => "!",
            K::MarkQuestion => "?",
            K::Pipe => "|",
            K::SlashForward => "/",
            K::SlashBackward => "\\",
            K::Percent => "%",
            K::And => "&",
            K::Equals => "=",
            K::Star => "*",
            K::Plus => "+",
            K::Minus => "-",
            K::Underscore => "_",
            K::Comma => ",",
            K::Colon => ":",
            K::ColonSemi => ";",
            K::Period => ".",
            K::Tilde => "~",
            K::Hashtag => "#",
            K::Caret => "^",
            K::Degree => "°",
            K::At => "@",
            K::CommentLine => "//",
            K::CommentStart => "/*",
            K::CommentEnd => "*/",
            K::EqualsDouble => "==",
            K::EqualsLess => "<=",
            K::EqualsMore => ">=",
            K::EqualsNot => "!=",
            K::EqualsStar => "*=",
            K::EqualsSlash => "/=",
            K::EqualsPlus => "+=",
            K::EqualsMinus => "-=",
            K::EqualsPercent => "%=",
            K::EqualsAnd => "&=",
            K::EqualsPipe => "|=",
            K::EqualsTide => "~=",
            K::EqualsCaret => "^=",
            K::EqualsMarkQuestion => "?=",
            K::EqualsHashtag => "#=",
            K::EqualsDegree => "°=",
            K::EqualsAt => "@=",
            K::ShiftLeft => "<<",
            K::ShiftRight => ">>",
            K::AndDouble => "&&",
            K::PipeDouble => "||",
            K::Arrow => "->",
            K::ArrowFat => "=>",
            K::ColonDoubleDouble => "::",
            K::EqualsShiftLeft => "<<=",
            K::EqualsShiftRight => ">>=",
            K::EqualsAndDouble => "&&=",
            K::EqualsPipeDouble => "||=",
            K::Namespace => "namespace",
            K::Import => "import",
            K::Public => "public",
            K::Local => "local",
            K::Const => "const",
            K::Mut => "mut",
            K::Struct => "struct",
            K::Var => "var",
            K::Unsafe => "unsafe",
            K::Extern => "extern",
            K::Inline => "inline",
            K::Interface => "interface",
            K::Type => "type",
            K::As => "as",
            K::Fn => "fn",
            K::Break => "break",
            K::Continue => "continue",
            K::If => "if",
            K::Else => "else",
            K::Switch => "switch",
            K::For => "for",
            K::While => "while",
            K::Return => "return",
            K::Enum => "enum",
            K::Union => "union",
            K::Static => "static",
            K::Staged => "staged",
            K::Yield => "yield",
            K::In => "in",
            K::Unsigned => "unsigned",
            K::With => "with",
            K::Where => "where",
            K::Super => "super",
            K::Satisfies => "satisfies",
            K::Alloc => "alloc",
            K::Implement => "implement",
            K::Ref => "ref",
            K::_Self => "Self",
            K::__Line => "__line",
            K::__File => "__file",
        }
    }
}

impl Token for KeywordToken {
    fn raw(&self) -> &str {
        self.keyword.raw()
    }

    fn source_position(&self) -> SourcePosition {
        self.source_position
    }
}

impl KeywordToken {
    pub fn new(keyword: Keyword, source_position: SourcePosition) -> Self {
        Self {
            keyword,
            source_position,
        }
    }

    pub fn keyword(&self) -> Keyword {
        self.keyword
    }
}

impl Token for IdentifierToken {
    fn raw(&self) -> &str {
        &self.raw
    }

    fn source_position(&self) -> SourcePosition {
        self.source_position
    }
}

impl IdentifierToken {
    pub fn new(raw: Box<str>, source_position: SourcePosition) -> Self {
        Self {
            raw,
            source_position,
        }
    }
}

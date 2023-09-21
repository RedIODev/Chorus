use std::{
    fmt::Display,
    fs::File,
    io::BufReader,
    io::{BufRead, Lines},
};

use strum::{IntoEnumIterator, VariantNames};

use crate::{error::LexerError, helper::PausableIterAdapter};

use super::{
    tokenizer, CodeSource, CommentToken, CommentType, FileCodeSource, FileCodeSourceImpl,
    IdentifierToken, Keyword, KeywordToken, SourcePosition, Token, TokenResult, TokenStream,
    TokenTrait,
};

impl SourcePosition {
    pub fn new(row: u32, column: u32) -> Self {
        Self { line: row, column }
    }

    pub(crate) fn new_line(&mut self) {
        self.line += 1;
        self.column = 0;
    }
}

impl From<(u32, u32)> for SourcePosition {
    fn from(value: (u32, u32)) -> Self {
        SourcePosition::new(value.0, value.1)
    }
}

impl Display for SourcePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.column + 1)
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
            K::Namespace => "namespace",
            K::Import => "import",
            K::Export => "export",
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
            K::Is => "is",
            K::Not => "not",
            K::Alloc => "alloc",
            K::Implement => "implement",
            K::Ref => "ref",
            K::_Self => "Self",
            K::__Line => "__line",
            K::__File => "__file",
        }
    }
}

impl TokenTrait for KeywordToken {
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

impl TokenTrait for IdentifierToken {
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

impl TokenTrait for CommentToken {
    fn raw(&self) -> &str {
        &self.raw
    }

    fn source_position(&self) -> SourcePosition {
        self.source_position
    }
}

impl CommentToken {
    pub fn new(comment_type: CommentType, raw: Box<str>, source_position: SourcePosition) -> Self {
        Self {
            comment_type,
            raw,
            source_position,
        }
    }
}

impl<CS> Iterator for TokenStream<CS>
where
    CS: CodeSource,
{
    type Item = Result<Token, CS::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        tokenizer::next_token(self).into()
    }
}

impl<CS> TokenStream<CS>
where
    CS: CodeSource,
{
    pub fn new(iter: CS::Iter) -> Self {
        Self {
            iter: iter.pausable_iter(),
            current_pos: SourcePosition::default(),
        }
    }
}

impl FileCodeSource for FileCodeSourceImpl {
    fn filename(&self) -> &str {
        todo!()
    }
}

impl<E> From<Option<Token>> for TokenResult<E> {
    fn from(value: Option<Token>) -> Self {
        match value {
            Some(t) => Self::Some(t),
            None => Self::None,
        }
    }
}

impl<E> From<TokenResult<E>> for Option<Result<Token, E>> {
    fn from(val: TokenResult<E>) -> Self {
        match val {
            TokenResult::Some(t) => Some(Ok(t)),
            TokenResult::None => None,
            TokenResult::Err(e) => Some(Err(e)),
        }
    }
}

impl From<KeywordToken> for Token {
    fn from(value: KeywordToken) -> Self {
        Self::Keyword(value)
    }
}

impl From<IdentifierToken> for Token {
    fn from(value: IdentifierToken) -> Self {
        Self::Identifier(value)
    }
}

impl From<CommentToken> for Token {
    fn from(value: CommentToken) -> Self {
        Self::Comment(value)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}[{}]@{}",
            Token::VARIANTS[self.varient()],
            self.raw(),
            self.source_position()
        )
    }
}

impl TokenTrait for Token {
    fn raw(&self) -> &str {
        match self {
            Token::Keyword(k) => k.raw(),
            Token::Identifier(i) => i.raw(),
            Token::Comment(c) => c.raw(),
        }
    }

    fn source_position(&self) -> SourcePosition {
        match self {
            Token::Keyword(k) => k.source_position(),
            Token::Identifier(i) => i.source_position(),
            Token::Comment(c) => c.source_position(),
        }
    }
}

impl Token {
    pub fn varient(&self) -> usize {
        unsafe { *(self as *const Self as *const usize) }
    }
}

impl CodeSource for FileCodeSourceImpl {
    type Iter = Lines<BufReader<File>>;

    type Error = std::io::Error;

    fn desc(&self) -> &str {
        todo!()
    }

    fn iter(&self) -> Result<TokenStream<Self>, Self::Error>
    where
        Self: Sized,
    {
        let ts = TokenStream::new(BufReader::new(self.file.try_clone()?).lines());
        Ok(ts)
    }
}

impl FileCodeSourceImpl {
    pub fn new(file: File) -> Self {
        Self { file }
    }
}

// impl CodeSource for FileCodeSourceImpl {
//     type Error = std::io::Error;

//     fn desc(&self) -> &str {
//         todo!()
//     }

//     // fn line_iter(&self) -> Self::Iter {
//     //     BufReader::new(self.filepath).lines().map(|r| r.map(String::as_str))
//     // }
// }

// impl Iterator for FileCodeSourceImpl {
//     type Item = Result<String, std::io::Error>;

//     fn next(&mut self) -> Option<Self::Item> {

//     }
// }

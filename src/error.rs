use err_derive::Error;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error(display = "error in lexer")]
    LexerError(#[error(source)] LexerError),
}

#[derive(Debug, Error)]
pub enum LexerError {
    #[error(display = "couldn't find keyword[{}]", _0)]
    KeywordNotFound(String),
}

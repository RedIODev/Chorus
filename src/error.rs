use err_derive::Error;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error(display = "error in lexer: ")]
    LexerError(#[error(source)] LexerError),
    #[error(display = "error in ast: ")]
    AstError(#[error(source)] AstError),
}

#[derive(Debug, Error)]
pub enum LexerError {
    #[error(display = "couldn't find keyword[{}]", _0)]
    KeywordNotFound(String),
}

#[derive(Debug, Error)]
pub enum AstError {
    #[error(display = "error while borrowing: ")]
    BorrowError(#[error(source)] BorrowError),
}

#[derive(Debug, Error)]
pub enum BorrowError {
    #[error(display = "cannot mutably borrow. Value is already borrowed imutably")]
    InvalidMutableBorrow,
    #[error(display = "cannot mutably borrow. Value is already borrowed mutably")]
    InvalidSecondMutableBorrow,
    #[error(display = "cannot imutably borrow. Value is already borrowed mutably")]
    InvalidImutableBorrow,
    #[error(display = "cannot imutably unborrow. Value is not borrowed")]
    InvalidUnborrow,
    #[error(display = "cannot mutably unborrow. Value is not borrowed mutably")]
    InvalidMutableUnborrow,
    
    #[error(display = "borrow mut error:")]
    BorrowMutError(#[error(source)] std::cell::BorrowMutError),
    #[error(display = "borrow error:")]
    BorrowError(#[error(source)] std::cell::BorrowError),
}

use std::collections::HashMap;
use std::ops::{Add, AddAssign};
use std::process::exit;

use std::{fs::File, path::Path};

use itertools::{Itertools, MultiPeek};
use lazy_static::lazy_static;
use memmap2::{Mmap, MmapOptions};
use num_enum::TryFromPrimitive;
use strum::VariantArray;
use strum_macros::{IntoStaticStr, VariantArray};

use crate::TmpResult;

pub struct Tokenizer {
    iter: MultiPeek<MmapReader>,
    line: u32,
    column: u32,
}

impl Tokenizer {
    pub fn new(file: impl AsRef<Path>) -> TmpResult<Tokenizer> {
        Ok(Tokenizer {
            iter: Self::open_file(file)?,
            line: 1,
            column: 1,
        })
    }

    pub fn next_token(&mut self) -> (Option<Token>, Whitespace) {
        let mut whitespace = Whitespace::default();
        whitespace += self.skip_whitespace();
        self.iter.reset_peek();
        let Some(first_char) = self.iter.peek().cloned() else {
            return (None, whitespace);
        };
        if let Ok(Ok(symbol)) = u8::try_from(first_char).map(Symbol::try_from_primitive) {
            self.iter.next();
            if let Some(token) = self.parse_comment(symbol) {
                return (Some(token), whitespace);
            }
            return (Some(Token::Symbol(symbol)), whitespace);
        }

        if let Some(keyword) = self.parse_keyword() {
            return (Some(Token::Keyword(keyword)), whitespace);
        }

        if !first_char.is_ascii_alphanumeric() {
            let line = self.line + whitespace.new_lines;
            let column = if whitespace.new_lines > 0 {
                0
            } else {
                self.column
            } + whitespace.columns;
            eprintln!("Syntax Error: Invalid symbol: '{first_char}'@{line}:{column}");
            exit(-1);
        }
        let identifier = self.parse_identifier();
        (Some(Token::Identifier(identifier)), whitespace)
    }

    fn open_file(filename: impl AsRef<Path>) -> TmpResult<MultiPeek<MmapReader>> {
        let file = File::open(filename)?;
        let mmap =  unsafe { MmapOptions::new().populate().map(&file)? };
        #[cfg(unix)]
        mmap.advise(memmap2::Advice::Sequential)?;
        //let reader = io::BufReader::new(file);
        Ok(MmapReader::new(mmap).multipeek())
    }

    fn skip_whitespace(&mut self) -> Whitespace {
        self.iter.reset_peek();
        let mut whitespace = Whitespace::default();
        while let Some(character) = self.iter.peek().copied() {
            if !character.is_whitespace() {
                break;
            }
            self.iter.next();
            whitespace.columns += 1;
            if character == '\n' {
                whitespace.new_lines += 1;
                whitespace.columns = 0;
            }
        }
        whitespace
    }

    fn parse_identifier(&mut self) -> String {
        self.iter.reset_peek();
        let mut identifier = String::new();
        while let Some(character) = self.iter.peek().copied() {
            if character.is_whitespace() {
                break;
            }
            if Symbol::try_from_primitive(character as u8).is_ok() {
                break;
            }
            self.iter.next();
            identifier.push(character);
        }

        identifier
    }

    fn parse_keyword(&mut self) -> Option<Keyword> {
        self.iter.reset_peek();
        let mut keyword = String::new();
        while let Some(character) = self.iter.peek().copied() {
            if !character.is_alphabetic() {
                break;
            }
            keyword.push(character);
        }
        let len = keyword.len();
        let keywords = KEYWORD_LEN_TABLE.get(&(len as u8))?;
        let keyword = keywords.get(&*keyword).copied();
        if keyword.is_some() {
            self.iter.nth(len-1);
        }
        keyword

    }

    fn parse_comment(&mut self, first_symbol: Symbol) -> Option<Token> {
        if !matches!(first_symbol, Symbol::Slash) {
            return None;
        }
        let second_char = self.iter.next()?;
        let second_symbol = Symbol::try_from_primitive(second_char as u8).ok()?;
        let comment = match second_symbol {
            Symbol::Slash => self.parse_line_comment(),
            Symbol::Asterisk => self.parse_block_comment(),
            _ => return None,
        };
        Some(comment)
    }

    fn parse_line_comment(&mut self) -> Token {
        self.iter.reset_peek();
        if let Some(third_char) = self.iter.peek().copied() 
                && let Some(third_symbol) = Symbol::try_from_primitive(third_char as u8).ok() 
                && matches!(third_symbol, Symbol::Slash) {
            self.iter.next();
            return self.parse_doc_comment();
        }
        

        let mut comment = String::from("//");
        for character in self.iter.by_ref() {
            if character == '\n' {
                break;
            }
            comment.push(character);
        }
        Token::Comment(Comment::Line(comment))
    }

    fn parse_doc_comment(&mut self) -> Token {
        self.iter.reset_peek();
        let mut comment = String::from("///");
        let mut newlines = 0;
        let mut found_newline = false;
        let mut slash_count = 0;
        while let Some(character) = self.iter.peek().copied() {
            comment.push(character);

            match character {
                '\n' => {
                    newlines+=1;
                    found_newline = true;
                    slash_count = 0;
                }
                '/' if found_newline => {
                    slash_count += 1;
                    if slash_count == 3 {
                        slash_count = 0;
                        found_newline = false;
                    }
                }
                c if c.is_whitespace() => {},
                _ if found_newline => {
                    comment.pop();
                    break;
                }
                 
                _ => {}
            }
            self.iter.next();
        }
        self.iter.reset_peek();
        Token::Comment(Comment::Doc(comment, newlines))
    }

    fn parse_block_comment(&mut self) -> Token {
        let mut comment = String::from("/*");
        let mut new_line_count = 0;
        let mut columns = 0;
        let mut found_asterisk = false;
        for character in self.iter.by_ref() {
            comment.push(character);
            columns += 1;

            match character {
                '\n' => {
                    new_line_count += 1;
                    columns = 0;
                }
                '*' => {
                    found_asterisk = true;
                    continue;
                }
                '/' if found_asterisk => break,
                _ => {}
            }
            found_asterisk = false;
        }

        Token::Comment(Comment::Block {
            comment,
            new_line_count,
            columns,
        })
    }
}

impl Iterator for Tokenizer {
    type Item = PosToken;

    fn next(&mut self) -> Option<Self::Item> {
        let (token, whitespace) = self.next_token();
        self.line += whitespace.new_lines;
        self.column += whitespace.columns;
        if whitespace.new_lines > 0 {
            self.column = whitespace.columns + 1;
        }
        let token = token?;
        let pos_token = PosToken {
            token,
            line: self.line,
            column: self.column,
        };
        self.line += pos_token.token.new_lines();
        self.column += pos_token.token.line_len();
        if pos_token.token.new_lines() > 0 {
            self.column = pos_token.token.line_len();
        }

        Some(pos_token)
    }
}

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

    fn new_lines(&self) -> u32 {
        match &self {
            Token::Comment(Comment::Line(_)) => 1,
            Token::Comment(Comment::Block { new_line_count, .. }) => *new_line_count,
            _ => 0,
        }
    }
}

lazy_static! {
    static ref KEYWORD_LEN_TABLE: HashMap<u8, HashMap<&'static str, Keyword>> = Keyword::VARIANTS 
        .iter()
        .copied()
        .chunk_by(|k| <&'static str>::from(*k).len() as u8)
        .into_iter()
        .map(|(k,g)| (k, g.map(|keyword| (<&'static str>::from(keyword), keyword)).collect()))
        .collect();
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

#[derive(Clone, Copy, Default)]
pub struct Whitespace {
    new_lines: u32,
    columns: u32,
}

impl Add for Whitespace {
    type Output = Whitespace;

    fn add(self, rhs: Self) -> Self::Output {
        Whitespace {
            new_lines: self.new_lines + rhs.new_lines,
            columns: self.columns + rhs.columns,
        }
    }
}

impl AddAssign for Whitespace {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

pub struct MmapReader {
    file:Mmap,
    byte_index:usize
}

impl MmapReader {

    pub fn new(file: Mmap) -> Self {
        Self { file, byte_index: 0 }
    }

    fn utf8_to_char(slice:&[u8]) -> Result<char, UTF8ReadError> {
        let chunk = slice.utf8_chunks().next().ok_or(UTF8ReadError::Eof)?;
        chunk.valid().chars().next().ok_or(UTF8ReadError::InvalidByte)
    }


}

impl Iterator for MmapReader {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let end_index = (self.file.len() - self.byte_index).clamp(0, 4) + self.byte_index;
            let next_slice = self.file.get(self.byte_index..end_index)?;
            match Self::utf8_to_char(next_slice) {
                Ok(char) => {
                    self.byte_index+=char.len_utf8();
                    return Some(char)
                },
                Err(UTF8ReadError::Eof) => return None,
                Err(UTF8ReadError::InvalidByte) => self.byte_index+=1
            }
        }
    }
}

enum UTF8ReadError {
    Eof,
    InvalidByte
}
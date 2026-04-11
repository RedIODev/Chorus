use std::collections::{BTreeMap, HashMap};
use std::iter::{FilterMap, Peekable};
use std::ops::{Add, AddAssign};
use std::{fs::File, path::Path};
use std::io::{self, BufRead, Bytes, Cursor, Read};

use enum_assoc::Assoc;
use itertools::{Itertools, MultiPeek, PeekNth, PeekingNext};
use lazy_static::lazy_static;
use num_enum::TryFromPrimitive;
use strum::VariantArray;
use strum_macros::{IntoStaticStr, VariantArray};
use unicode_reader::CodePoints;

use crate::TmpResult;

pub struct Tokenizer {
    chars: PeekNth<Characters>,
    line: u32,
    column: u32,
    peek_index: u32
}

impl Tokenizer {
    pub fn new(file: impl AsRef<Path>) -> TmpResult<Tokenizer> {
        Ok(Tokenizer { chars: read_chars(file)?, line: 1, column: 1, peek_index: 0 })
    }

    /// Peeks the found whitespace and the token at the end of it if found.
    /// Whitespace after tokens is not included. 
    pub fn peek(&mut self) -> (Option<Token>, Whitespace) {
        let mut whitespace = Whitespace::EMPTY;
        whitespace += self.skip_whitespace();
        let Some(first_char) = self.chars.peek_nth(self.peek_index as usize).cloned() else { return (None, whitespace) };
        if let Ok(symbol) = Symbol::try_from_primitive(first_char as u8) {
            if let Some(token) = self.parse_comment(symbol) {
                return (Some(token), whitespace);
            }
            self.peek_index +=1;
            return (Some(Token::Symbol(symbol)), whitespace);
        }

        if let Some(keyword) = self.parse_keyword() {
            return (Some(Token::Keyword(keyword)), whitespace);
        }

        if !first_char.is_ascii_alphanumeric() {
            panic!("Syntax Error: Invalid symbol:'{first_char}'")
        }
        let identifier = self.parse_identifier();
        (Some(Token::Identifier(identifier)), whitespace)
    }

    fn skip_whitespace(&mut self) -> Whitespace {
        let mut whitespace = Whitespace::EMPTY;
        while let Some(character) = self.chars.peek_nth(self.peek_index as usize) {
            if !character.is_whitespace() { break; }
            self.peek_index+=1;
            whitespace.columns += 1;
            if *character == '\n' { 
                whitespace.new_lines += 1; 
                whitespace.columns = 0;
            }
        }
        whitespace
    }

    fn parse_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(character) = self.chars.peek_nth(self.peek_index as usize) {
            if character.is_whitespace() { break;}
            if Symbol::try_from_primitive(*character as u8).is_ok() {break;}
            self.peek_index+=1;
            identifier.push(*character);
        }

        identifier
    }

    fn parse_keyword(&mut self) -> Option<Keyword> {
        let mut string = String::new();
        for i in 0..*KEYWORD_MAX_LEN {
            if let Some(character) = self.chars.peek_nth(i + self.peek_index as usize) {
                string.push(*character);
            }
        }
        let keyword = KEYWORD_MAP.iter().find(|(k,_)| string.starts_with(*k));
        let Some((name, keyword)) = keyword else {return None};
        self.peek_index += name.len() as u32;
        Some(*keyword)
    }

    fn parse_comment(&mut self, first_symbol: Symbol) -> Option<Token> {
        if !matches!(first_symbol, Symbol::Slash) {return None;}
        let second_char = *self.chars.peek_nth(self.peek_index as usize + 1)?;
        let second_symbol = Symbol::try_from_primitive(second_char as u8).ok()?;
        self.peek_index +=2;
        let comment = match second_symbol {
            Symbol::Slash => self.parse_line_comment(),
            Symbol::Asterisk => self.parse_block_comment(),
            _ => return None
        };
        Some(comment)
    }

    fn parse_line_comment(&mut self) -> Token {
        let mut comment = String::from("//");
        while let Some(character) = self.chars.peek_nth(self.peek_index as usize) {
            self.peek_index+=1;
            if *character == '\n' {break;}
            comment.push(*character);
        }
        Token::Comment(Comment::LineComment(comment))
    }

    fn parse_block_comment(&mut self) -> Token {
        let mut comment = String::from("/*");
        let mut new_line_count = 0;
        let mut columns = 0;
        while let Some(character) = self.chars.peek_nth(self.peek_index as usize) {
            self.peek_index+=1;
            comment.push(*character);
            columns+=1;
            if *character == '\n' { 
                new_line_count+=1;
                columns = 0;
            };
            if *character != '*' {continue;}
            let Some(next_char) = self.chars.peek_nth(self.peek_index as usize) else {break;};
            if *next_char == '/' {break;}
        }

        Token::Comment(Comment::BlockComment {comment, new_line_count, columns})
    }
}

impl Iterator for Tokenizer {
    type Item = PosToken;

    fn next(&mut self) -> Option<Self::Item> {
        let (token, whitespace) = self.peek();
        self.line += whitespace.new_lines;
        self.column += whitespace.columns;
        if whitespace.new_lines > 0 {
            self.column = whitespace.columns;
        }
        let token = token?;
        let pos_token = PosToken {token, line: self.line, column: self.column };
        self.line += pos_token.token.new_lines();
        self.column += pos_token.token.line_len();
        if pos_token.token.new_lines() > 0 {
            self.column = pos_token.token.line_len();
        }

        Some(pos_token)
    }
}

type Characters = FilterMap<CodePoints<Bytes<io::BufReader<File>>>, fn (Result<char, io::Error>) -> Option<char>>;

fn read_chars(filename: impl AsRef<Path>) -> io::Result<PeekNth<Characters>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let characters: Characters = CodePoints::from(reader.bytes()).filter_map(Result::ok);
    
    Ok(itertools::peek_nth(characters))
    
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Symbol(Symbol),
    Comment(Comment),
}

impl Token {
    fn line_len(&self) -> u32 {
        match &self {
            Token::Comment(Comment::LineComment(_)) => 0,
            Token::Comment(Comment::BlockComment { columns, .. }) => *columns,
            Token::Identifier(identifier) => identifier.len() as u32,
            Token::Keyword(keyword) => <&str>::from(keyword).len() as u32,
            Token::Symbol(_) => 1
        }
    }

    fn new_lines(&self) -> u32 {
        match &self {
            Token::Comment(Comment::LineComment(_)) => 1,
            Token::Comment(Comment::BlockComment{new_line_count, ..}) => *new_line_count, 
            _ => 0
        }
    }
}

lazy_static! {
    static ref KEYWORD_MAP:BTreeMap<&'static str, Keyword> = Keyword::VARIANTS.iter().map(|keyword| (<&'static str>::from(keyword), *keyword)).collect();
    static ref KEYWORD_MAX_LEN:usize =KEYWORD_MAP.keys().max_by_key(|name| name.len()).expect("unreachable!").len();
}

#[derive(Debug)]
pub enum Comment {
    LineComment(String),
    BlockComment {
        comment:String,
        new_line_count:u32,
        columns:u32,
    }
}

impl Comment {
    pub fn comment(&self) -> &str {
        match self {
            Comment::BlockComment{ comment, ..} => comment,
            Comment::LineComment(comment) => comment
        }
    }
}

#[derive(IntoStaticStr, VariantArray, Clone, Copy, Debug)]
#[strum(serialize_all="lowercase")]
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
    Degree = b'\xB0',// "°" ASCII character
    Backslash = b'\\',
    Percent = b'%',
    Dollar = b'$',
    At = b'@'
}

impl Symbol {
    pub const fn char_equals(self, character:char) -> bool {
        self as u8 == character as u8
    }
}
#[derive(Debug)]
pub struct PosToken {
    pub token: Token,
    pub line: u32,
    pub column: u32,
}



#[derive(Clone, Copy)]
pub struct Whitespace {
    new_lines:u32,
    columns:u32
}

impl Add for Whitespace {
    type Output = Whitespace;

    fn add(self, rhs: Self) -> Self::Output {
        Whitespace { new_lines: self.new_lines+rhs.new_lines, columns: self.columns + rhs.columns }
    }
}

impl AddAssign for Whitespace {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Whitespace {
    const EMPTY:Whitespace = Whitespace {new_lines: 0, columns: 0 };
}
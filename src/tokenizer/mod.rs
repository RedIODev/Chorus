use std::collections::HashMap;
use std::ops::{Add, AddAssign};
use std::process::exit;

use std::{fs::File, path::Path};

use itertools::{Itertools, MultiPeek};
use lazy_static::lazy_static;
use memmap2::MmapOptions;
use num_enum::TryFromPrimitive;
use strum::VariantArray;

use crate::TmpResult;
use crate::tokenizer::mmap_reader::MmapReader;
use crate::tokenizer::token_types::{Comment, Keyword, PosToken, Symbol, Token};

mod mmap_reader;
mod token_types;

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
    
    fn open_file(filename: impl AsRef<Path>) -> TmpResult<MultiPeek<MmapReader>> {
        let file = File::open(filename)?;
        let mmap =  unsafe { MmapOptions::new().populate().map(&file)? };
        #[cfg(unix)]
        mmap.advise(memmap2::Advice::Sequential)?;
        Ok(MmapReader::new(mmap).multipeek())
    }

    fn next_token(&mut self) -> (Option<Token>, Whitespace) {
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

lazy_static! {
    static ref KEYWORD_LEN_TABLE: HashMap<u8, HashMap<&'static str, Keyword>> = Keyword::VARIANTS 
        .iter()
        .copied()
        .chunk_by(|k| <&'static str>::from(*k).len() as u8)
        .into_iter()
        .map(|(k,g)| (k, g.map(|keyword| (<&'static str>::from(keyword), keyword)).collect()))
        .collect();
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


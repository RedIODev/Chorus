use lazy_static::lazy_static;
use unicode_segmentation::UnicodeSegmentation;

use crate::{helper::Strlen, lexer::KeywordToken};

use super::{
    CodeSource, CommentToken, CommentType, IdentifierToken, Keyword, SourcePosition, Token,
    TokenResult, TokenStream,
};
use strum::{EnumCount, IntoEnumIterator};

fn ordered_keywords() -> impl Iterator<Item = Keyword> {
    lazy_static! {
        static ref LIST: [Keyword; Keyword::COUNT] = {
            let mut vec = Keyword::iter().collect::<Vec<_>>();
            vec.sort_by_key(|k2| std::cmp::Reverse(k2.raw().chars().count()));
            vec.try_into()
                .expect("Keyword::COUNT != Keyword::iter().count() should be unreachable")
        };
    }
    LIST.iter().copied()
}

fn ordered_symbols() -> impl Iterator<Item = Keyword> {
    ordered_keywords().filter(|keyword| !keyword.raw().contains(char::is_alphanumeric))
}

pub(super) fn next_token<CS>(stream: &mut TokenStream<CS>) -> TokenResult<CS::Error>
where
    CS: CodeSource,
{
    for line in &mut stream.iter {
        let line_ok = match line {
            Ok(ref line) => line.as_str(),
            Err(e) => return TokenResult::Err(e),
        };
        let token = get_token_from_line(line_ok, &mut stream.current_pos);
        if let Some(token) = token {
            stream.iter.pause(line);
            return Some(token).into();
        }
        stream.current_pos.new_line();
    }
    None.into()
}

fn get_token_from_line(mut line: &str, pos: &mut SourcePosition) -> Option<Token> {
    let line_org = line;
    loop {
        line = &line_org[pos.column as usize..];
        if line.is_empty() {
            return None;
        }
        if let Some(len) = starts_with_whitespace(line) {
            pos.column += len as u32;
            continue;
        }

        if starts_with_line_comment(line) {
            let current_pos = *pos;
            pos.column += line.len() as u32;
            return Some(
                CommentToken::new(
                    CommentType::LineComment,
                    Box::from(line.trim_end()),
                    current_pos,
                )
                .into(),
            );
        }

        if let Some(keyword) = starts_with_keyword(line) {
            let current_pos = *pos;
            pos.column += keyword.raw().len() as u32;
            return Some(KeywordToken::new(keyword, current_pos).into());
        }

        let mut end = 0;

        while starts_with_identifier(&line[end..]) {
            end += 1;
        }
        let current_pos = *pos;
        pos.column += end as u32;
        return Some(IdentifierToken::new(Box::from(line[..end].trim_end()), current_pos).into());
    }
}

fn starts_with_identifier(line: &str) -> bool {
    !line.is_empty() && starts_with_whitespace(line).is_none() && starts_with_symbol(line).is_none()
}

fn starts_with_whitespace(line: &str) -> Option<usize> {
    if let Some(grapheme) = line.graphemes(true).next() {
        if grapheme.contains(char::is_whitespace) {
            return Some(grapheme.len());
        }
    }
    None
}

const LINE_COMMENT: &str = "//";

fn starts_with_line_comment(line: &str) -> bool {
    line.starts_with(LINE_COMMENT)
}

fn starts_with_keyword(line: &str) -> Option<Keyword> {
    starts_with_keyword_from_iter(line, ordered_keywords())
}

fn starts_with_symbol(line: &str) -> Option<Keyword> {
    starts_with_keyword_from_iter(line, ordered_symbols())
}

fn starts_with_keyword_from_iter(
    line: &str,
    keyword_iter: impl Iterator<Item = Keyword>,
) -> Option<Keyword> {
    for keyword in keyword_iter {
        let raw_keyword = keyword.raw();
        if line.strlen() < raw_keyword.strlen() {
            continue;
        }
        if line.starts_with(raw_keyword) {
            return Some(keyword);
        }
    }
    None
}

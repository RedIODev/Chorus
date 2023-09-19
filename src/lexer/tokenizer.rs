use lazy_static::lazy_static;
use unicode_segmentation::UnicodeSegmentation;

use crate::{helper::Strlen, lexer::KeywordToken};

use super::{CodeSource, IdentifierToken, Keyword, SourcePosition, Token, TokenStream, TokenResult};
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
    ordered_keywords()
    .filter(|keyword| !keyword.raw().contains(char::is_alphanumeric))
}

pub(super) fn next_token<CS>(stream: &mut TokenStream<CS>) -> TokenResult<CS::Error>
where
    CS: CodeSource,
{
    for line in &mut stream.iter {
        let line_ok = match line {
            Ok(ref line) => line.as_str(),
            Err(e) => return TokenResult::Err(e)
        };
        let token = get_token_from_line(line_ok, &mut stream.current_pos);
        if let Some(token) = token {
            stream.iter.pause(line);
            return Some(token).into();
        }
        stream.current_pos.line += 1;
        stream.current_pos.column = 0;
    }
    None.into()
}

fn get_token_from_line(mut line: &str, pos: &mut SourcePosition) -> Option<Box<dyn Token>> {
    let line_org = line;
    while !line.is_empty() {//line problem here. rethink slicing of line (infinite loop incorrenct condition)
        line = &line_org[pos.column as usize..];
        println!("line:[{}] len:{}", line, line.len());
        if let Some(len) = starts_with_whitespace(line) {
            pos.column += len as u32;
            continue;
        }
        if let Some(keyword) = starts_with_keyword(line) {
            let current_pos = *pos;
            pos.column += keyword.raw().len() as u32;
            return Some(Box::new(KeywordToken::new(keyword, current_pos)));
        }
        let mut end = 0;
        while line.len() > end && starts_with_symbol(&line[end..]).is_none() {
            end += 1;
        }
        let current_pos = *pos;
        pos.column += end as u32;
        return Some(Box::new(IdentifierToken::new(
            Box::from(line[..end].trim_end()),
            current_pos,
        )));
    }
    None
}

fn starts_with_whitespace(line: &str) -> Option<usize> {
    if let Some(grapheme) = line.graphemes(true).next() {
        if grapheme.contains(char::is_whitespace) {
            return Some(grapheme.len());
        }
    }
    None
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

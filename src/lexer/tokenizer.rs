use lazy_static::lazy_static;

use super::{Keyword, Token};
use strum::{EnumCount, IntoEnumIterator};
pub fn tokenize(lines: &[&str]) -> Box<[Box<dyn Token>]> {
    let accumelator = Vec::new();
    for (line_number, line) in lines.iter().enumerate() {}
    accumelator.into_boxed_slice()
}

pub fn ordered_keywords() -> impl Iterator<Item = Keyword> {
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

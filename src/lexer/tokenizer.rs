use super::Token;

pub fn tokenize(lines: &[&str]) -> Box<[Box<dyn Token>]> {
    let accumelator = Vec::new();
    for (line_number, line) in lines.iter().enumerate() {

    }
    accumelator.into_boxed_slice()
}

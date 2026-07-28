//! Selector extension parser.

use super::pseudo::parse_pseudo;
use super::types::{normalized_base, SelectorPlan};

pub(crate) fn parse(source: &str) -> SelectorPlan {
    let mut base = String::new();
    let mut filters = Vec::new();
    let (mut index, mut bracket, mut paren) = (0usize, 0usize, 0usize);
    let mut quote = None;
    while index < source.len() {
        let ch = source[index..].chars().next().unwrap();
        if let Some(expected) = quote {
            push(&mut base, &mut index, ch);
            if ch == expected {
                quote = None;
            }
            continue;
        }
        match ch {
            '"' | '\'' => {
                quote = Some(ch);
                push(&mut base, &mut index, ch);
            }
            '[' => {
                bracket += 1;
                push(&mut base, &mut index, ch);
            }
            ']' if bracket > 0 => {
                bracket -= 1;
                push(&mut base, &mut index, ch);
            }
            '(' => {
                paren += 1;
                push(&mut base, &mut index, ch);
            }
            ')' if paren > 0 => {
                paren -= 1;
                push(&mut base, &mut index, ch);
            }
            ':' if bracket == 0 && paren == 0 => match parse_pseudo(source, index, &mut filters) {
                Some(next) => index = next,
                None => push(&mut base, &mut index, ch),
            },
            _ => push(&mut base, &mut index, ch),
        }
    }
    SelectorPlan::new(normalized_base(base), filters)
}

fn push(out: &mut String, index: &mut usize, ch: char) {
    out.push(ch);
    *index += ch.len_utf8();
}

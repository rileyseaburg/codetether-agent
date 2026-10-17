//! JSON-aware location and removal of complete tool-call blocks.

mod tail;

use super::markup;
use serde_json::{Deserializer, Value};
use std::ops::Range;

pub(super) struct Block<'a> {
    pub(super) range: Range<usize>,
    pub(super) json: &'a str,
}

pub(super) fn blocks(text: &str) -> Vec<Block<'_>> {
    let mut output = Vec::new();
    let mut cursor = 0;
    while let Some(start) = markup::call_start_regex().find_at(text, cursor) {
        let json_start = start.end();
        let mut values = Deserializer::from_str(&text[json_start..]).into_iter::<Value>();
        let Some(Ok(_)) = values.next() else {
            cursor = start.end();
            continue;
        };
        let json_end = json_start + values.byte_offset();
        let Some(end) = tail::end(text, json_end) else {
            cursor = start.end();
            continue;
        };
        output.push(Block {
            range: start.start()..end,
            json: &text[json_start..json_end],
        });
        cursor = end;
    }
    output
}

pub(super) fn remove(text: &str, ranges: &[Range<usize>]) -> String {
    let mut output = String::new();
    let mut cursor = 0;
    for range in ranges {
        output.push_str(&text[cursor..range.start]);
        cursor = range.end;
    }
    output.push_str(&text[cursor..]);
    output
}

#[cfg(test)]
#[path = "scan_tests.rs"]
mod tests;

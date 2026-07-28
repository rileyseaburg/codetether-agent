//! Pseudo selector token recognition.

use super::args;
use super::simple::parse_simple;
use super::types::SelectorFilter;
use super::types::SelectorFilter::*;

pub(crate) fn parse_pseudo(
    source: &str,
    index: usize,
    filters: &mut Vec<SelectorFilter>,
) -> Option<usize> {
    parse_simple(source, index, filters)
        .or_else(|| has_text(source, index, filters))
        .or_else(|| nth(source, index, filters))
}

fn has_text(source: &str, index: usize, filters: &mut Vec<SelectorFilter>) -> Option<usize> {
    functional(source, index, ":has-text(").map(|(value, end)| {
        filters.push(HasText(value));
        end
    })
}

fn nth(source: &str, index: usize, filters: &mut Vec<SelectorFilter>) -> Option<usize> {
    functional(source, index, ":nth(").map(|(value, end)| {
        filters.push(value.parse::<usize>().map(Nth).unwrap_or(Invalid));
        end
    })
}

fn functional(source: &str, index: usize, name: &str) -> Option<(String, usize)> {
    source[index..]
        .starts_with(name)
        .then(|| args::read(source, index + name.len()))
        .flatten()
}

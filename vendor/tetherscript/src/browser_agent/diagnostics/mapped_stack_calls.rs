//! Generated JavaScript call lookup helpers.

use super::mapped_stack_functions::FunctionRange;
use super::mapped_types::GeneratedSourceLocation;

pub fn containing(funcs: &[FunctionRange], index: usize) -> Option<&FunctionRange> {
    funcs
        .iter()
        .find(|item| item.body_start <= index && index <= item.body_end)
}

pub fn caller<'a>(
    source: &str,
    funcs: &'a [FunctionRange],
    callee: &str,
) -> Option<(&'a FunctionRange, usize)> {
    funcs.iter().find_map(|item| {
        if item.name == callee {
            return None;
        }
        super::mapped_stack_position::call(source, item.body_start, item.body_end, callee)
            .map(|index| (item, index))
    })
}

pub fn location(source: &str, script_url: &str, index: usize) -> GeneratedSourceLocation {
    let (line, column) = super::mapped_stack_position::line_column(source, index);
    GeneratedSourceLocation {
        script_url: script_url.into(),
        line,
        column,
    }
}

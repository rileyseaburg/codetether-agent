//! Generated JavaScript call-stack inference.

use super::mapped_types::GeneratedSourceLocation;

pub fn frames(
    source: &str,
    generated: &GeneratedSourceLocation,
) -> Vec<(Option<String>, GeneratedSourceLocation)> {
    let funcs = super::mapped_stack_functions::collect(source);
    let Some(index) = super::mapped_stack_position::index(source, generated.line, generated.column)
    else {
        return vec![(None, generated.clone())];
    };
    let mut out = Vec::new();
    let current =
        super::mapped_stack_calls::containing(&funcs, index).map(|item| item.name.clone());
    out.push((current.clone(), generated.clone()));
    super::mapped_stack_walk::append(source, &funcs, current, &generated.script_url, &mut out);
    out
}

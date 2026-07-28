//! Generated JavaScript caller stack walk.

use super::mapped_stack_functions::FunctionRange;
use super::mapped_types::GeneratedSourceLocation;

pub fn append(
    source: &str,
    funcs: &[FunctionRange],
    mut callee: Option<String>,
    script_url: &str,
    out: &mut Vec<(Option<String>, GeneratedSourceLocation)>,
) {
    let mut seen = Vec::new();
    while let Some(name) = callee.clone() {
        if seen.contains(&name) {
            break;
        }
        seen.push(name.clone());
        let Some((caller, index)) = super::mapped_stack_calls::caller(source, funcs, &name) else {
            break;
        };
        callee = Some(caller.name.clone());
        let location = super::mapped_stack_calls::location(source, script_url, index);
        out.push((callee.clone(), location));
    }
}

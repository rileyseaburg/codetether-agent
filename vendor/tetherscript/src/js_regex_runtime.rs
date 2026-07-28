//! Runtime helpers for the small JavaScript regex subset.

#[path = "js_regex_runtime/class.rs"]
mod class;
#[path = "js_regex_runtime/decimal.rs"]
mod decimal;
#[path = "js_regex_runtime/escaped.rs"]
mod escaped;
#[path = "js_regex_runtime/intrinsic_name.rs"]
mod intrinsic_name;
#[path = "js_regex_runtime/intrinsic_path.rs"]
mod intrinsic_path;
#[path = "js_regex_runtime/route.rs"]
mod route;
#[path = "js_regex_runtime/shorthand.rs"]
mod shorthand;
#[path = "js_regex_runtime/suffix.rs"]
mod suffix;

pub(crate) fn find(text: &str, pattern: &str) -> Option<(usize, usize)> {
    exec(text, pattern, "").map(|(start, end, _)| (start, end))
}

pub(crate) fn exec(
    text: &str,
    pattern: &str,
    flags: &str,
) -> Option<(usize, usize, Vec<Option<String>>)> {
    if let Some(found) = route::exec(text, pattern, flags) {
        return Some(found);
    }
    find_basic(text, pattern).map(|(start, end)| (start, end, Vec::new()))
}

fn find_basic(text: &str, pattern: &str) -> Option<(usize, usize)> {
    if pattern == "^is[A-Z]" {
        let mut chars = text.chars();
        let matched =
            chars.next()? == 'i' && chars.next()? == 's' && chars.next()?.is_ascii_uppercase();
        return matched.then_some((0, 3));
    }
    if let Some(found) = decimal::find(text, pattern) {
        return Some(found);
    }
    if let Some(found) = shorthand::find(text, pattern) {
        return Some(found);
    }
    if let Some(found) = escaped::repeat(text, pattern) {
        return Some(found);
    }
    if let Some(found) = escaped::single(text, pattern) {
        return Some(found);
    }
    if let Some(found) = intrinsic_path::find(text, pattern) {
        return Some(found);
    }
    if let Some(found) = intrinsic_name::find(text, pattern) {
        return Some(found);
    }
    if let Some(found) = suffix::find(text, pattern) {
        return Some(found);
    }
    if let Some(found) = class::find(text, pattern) {
        return Some(found);
    }
    text.find(pattern).map(|i| (i, i + pattern.len()))
}

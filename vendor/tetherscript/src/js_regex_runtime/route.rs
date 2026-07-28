//! React Router style anchored path regex matching.

#[path = "route/body.rs"]
mod body;
#[path = "route/literal.rs"]
mod literal;
#[path = "route/matcher.rs"]
mod matcher;
#[path = "route/segment.rs"]
mod segment;
#[path = "route/splat.rs"]
mod splat;

pub(super) fn exec(
    text: &str,
    pattern: &str,
    flags: &str,
) -> Option<(usize, usize, Vec<Option<String>>)> {
    let (body, end, has_splat) = body::parse(pattern)?;
    let mut index = 0;
    let mut captures = Vec::new();
    if !matcher::matches(text, body, flags.contains('i'), &mut index, &mut captures) {
        return None;
    }
    if has_splat {
        splat::finish(text, index, captures)
    } else if end {
        let tail = &text[index..];
        tail.chars()
            .all(|ch| ch == '/')
            .then_some((0, text.len(), captures))
    } else {
        (index == text.len() || text[index..].starts_with('/')).then_some((0, index, captures))
    }
}

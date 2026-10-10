//! Path and identifier syntax shared by routing and frame validation.

/// Lowercase hyphenated 36-character UUID syntax, as the TS relay checks.
pub(crate) fn uuid_like(id: &str) -> bool {
    id.len() == 36
        && id
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c) || c == b'-')
}
/// Split `/companion/sessions/{id}[/{action}]` with a known action.
pub(crate) fn session(path: &str) -> Option<(&str, Option<&str>)> {
    let rest = path.strip_prefix("/companion/sessions/")?;
    let (id, action) = rest
        .split_once('/')
        .map_or((rest, None), |(i, a)| (i, Some(a)));
    let known = ["events", "frames", "pause", "commands", "request"];
    let known = action.is_none_or(|a| known.contains(&a));
    (uuid_like(id) && known).then_some((id, action))
}

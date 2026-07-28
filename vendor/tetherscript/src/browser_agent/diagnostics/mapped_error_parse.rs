//! Error-message source location parsing helpers.

pub fn explicit_position(message: &str) -> Option<(usize, usize)> {
    let (_, tail) = message.rsplit_once(" at ")?;
    let (line, column) = tail.split_once(':')?;
    Some((line.parse().ok()?, column.parse().ok()?))
}

pub fn reference_name(message: &str) -> Option<String> {
    let rest = message.strip_prefix("ReferenceError: ")?;
    let (name, _) = rest.split_once(" is not defined")?;
    Some(name.to_string())
}

pub fn find(source: &str, name: &str) -> Option<(usize, usize)> {
    source.lines().enumerate().find_map(|(line, text)| {
        text.find(name)
            .map(|column| (line.saturating_add(1), column.saturating_add(1)))
    })
}

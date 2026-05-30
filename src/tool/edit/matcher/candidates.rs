use super::normalize;

pub fn whitespace_match(content: &str, old: &str) -> Option<String> {
    let wanted = normalize::line_key(old);
    windows(content, old.lines().count())
        .into_iter()
        .find(|candidate| normalize::line_key(candidate) == wanted)
}

pub fn nearest(content: &str, old: &str) -> Option<String> {
    windows(content, old.lines().count())
        .into_iter()
        .max_by_key(|candidate| normalize::score(candidate, old))
}

fn windows(content: &str, height: usize) -> Vec<String> {
    let lines: Vec<&str> = content.lines().collect();
    let wanted = height.max(1).min(lines.len().max(1));
    lines
        .windows(wanted)
        .map(|chunk| chunk.join("\n"))
        .collect()
}

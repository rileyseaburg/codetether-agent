pub fn tolerant_match<'a>(content: &'a str, old: &str) -> Option<&'a str> {
    let target = normalize(old);
    content
        .match_indices(first_token(old).unwrap_or(""))
        .filter_map(|(start, _)| block_at(content, start, old.lines().count()))
        .find(|candidate| normalize(candidate) == target)
}

pub fn nearest(content: &str, old: &str) -> Option<String> {
    let lines = old.lines().count().max(1);
    content
        .lines()
        .collect::<Vec<_>>()
        .windows(lines)
        .map(|w| w.join("\n"))
        .max_by_key(|candidate| score(&normalize(candidate), &normalize(old)))
}

fn block_at(content: &str, start: usize, lines: usize) -> Option<&str> {
    let mut end = start;
    for i in 0..lines {
        match content[end..].find('\n') {
            Some(j) if i + 1 < lines => end += j + 1,
            Some(j) => end += j,
            None => end = content.len(),
        }
    }
    content.get(start..end)
}

fn normalize(s: &str) -> String {
    s.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn first_token(s: &str) -> Option<&str> {
    s.split_whitespace().next()
}

fn score(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .filter(|(left, right)| left == right)
        .count()
}

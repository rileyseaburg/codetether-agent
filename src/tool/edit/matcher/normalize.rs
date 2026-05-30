pub fn line_key(text: &str) -> String {
    text.lines().map(str::trim).collect::<Vec<_>>().join("\n")
}

pub fn score(a: &str, b: &str) -> usize {
    let aa: Vec<&str> = a.split_whitespace().collect();
    let bb: Vec<&str> = b.split_whitespace().collect();
    aa.iter()
        .zip(bb.iter())
        .filter(|(left, right)| left == right)
        .count()
}

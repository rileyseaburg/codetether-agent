pub(crate) fn summarize_title(prompt: &str) -> String {
    const LIMIT: usize = 80;
    let trimmed = prompt.trim();
    if trimmed.chars().count() <= LIMIT {
        trimmed.to_string()
    } else {
        let mut title: String = trimmed.chars().take(LIMIT).collect();
        title.push_str("...");
        title
    }
}

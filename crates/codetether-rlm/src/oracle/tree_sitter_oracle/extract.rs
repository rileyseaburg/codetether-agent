use anyhow::Result;

const KEYWORDS: &[&str] = &["pub", "fn", "struct", "enum", "impl", "trait"];

pub(crate) fn struct_fields(body: &str) -> Result<Vec<String>> {
    capture_names(body, r"(?:pub\s+)?(\w+)\s*:", |_| true)
}

pub(crate) fn enum_variants(body: &str) -> Result<Vec<String>> {
    capture_names(body, r"(\w+)\s*(?:,|=|\{|\()", |name| {
        !KEYWORDS.contains(&name)
    })
}

fn capture_names(body: &str, pattern: &str, keep: impl Fn(&str) -> bool) -> Result<Vec<String>> {
    let re = regex::Regex::new(pattern)?;
    let names = re
        .captures_iter(body)
        .filter_map(|cap| cap.get(1))
        .map(|name| name.as_str())
        .filter(|name| keep(name))
        .map(str::to_string)
        .collect();
    Ok(names)
}

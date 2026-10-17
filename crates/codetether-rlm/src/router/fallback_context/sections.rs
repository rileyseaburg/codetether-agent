//! Append the formatted request and activity sections.

pub(super) fn append_sections(parts: &mut Vec<String>, head: &str, tail: &str) {
    parts.extend_from_slice(&[
        "### Initial Request".into(),
        "```".into(),
        head.into(),
        "```".into(),
        String::new(),
        "### Recent Activity".into(),
        "```".into(),
        tail.into(),
        "```".into(),
    ]);
}

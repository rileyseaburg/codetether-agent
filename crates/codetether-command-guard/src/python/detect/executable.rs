//! Match Python executable basenames, not arbitrary python-prefixed tools.

pub(super) fn python(word: &str) -> bool {
    let lower = word
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    let name = lower.strip_suffix(".exe").unwrap_or(&lower);
    let Some(version) = name.strip_prefix("python") else {
        return false;
    };
    let version = version.strip_prefix('w').unwrap_or(version);
    version.is_empty()
        || version.starts_with(|c: char| c.is_ascii_digit())
            && version.chars().all(|c| c.is_ascii_digit() || c == '.')
}

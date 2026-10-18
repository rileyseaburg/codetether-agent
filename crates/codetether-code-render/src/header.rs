//! Frame titles sized to the available terminal width.

pub(crate) fn header(language: &str, width: usize) -> String {
    let title = if language.is_empty() {
        "┌─ Code ─".to_string()
    } else {
        format!("┌─ {language} Code ─")
    };
    let pad = width.saturating_sub(title.chars().count());
    format!("{title}{}", "─".repeat(pad))
}

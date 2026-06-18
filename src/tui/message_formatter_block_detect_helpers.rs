//! Pure detectors for block-level markdown constructs.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

pub(super) fn header_spans(hashes: usize, text: &str) -> Vec<Span<'static>> {
    let (marker, color) = match hashes {
        1 => ("▌ ", Color::Cyan),
        2 => ("▎ ", Color::Cyan),
        _ => ("› ", Color::Blue),
    };
    vec![
        Span::styled(marker, Style::default().fg(color)),
        Span::styled(
            text.to_string(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ),
    ]
}

pub(super) fn is_rule(t: &str) -> bool {
    matches!(t, "***" | "___") || (t.len() >= 3 && t.chars().all(|c| c == '-'))
}
pub(super) fn header(t: &str) -> Option<(usize, String)> {
    let hashes = t.chars().take_while(|c| *c == '#').count();
    if (1..=6).contains(&hashes) && t.chars().nth(hashes) == Some(' ') {
        return Some((hashes, t[hashes + 1..].to_string()));
    }
    None
}

pub(super) fn blockquote(t: &str) -> Option<String> {
    t.strip_prefix("> ")
        .or_else(|| t.strip_prefix(">"))
        .map(str::to_string)
}

pub(super) fn list_item(t: &str) -> Option<(String, String)> {
    for m in ["- ", "* ", "+ "] {
        if let Some(rest) = t.strip_prefix(m) {
            return Some(("•".to_string(), rest.to_string()));
        }
    }
    let digits = t.chars().take_while(char::is_ascii_digit).count();
    let after = t.get(digits..).filter(|_| digits > 0)?;
    let rest = after
        .strip_prefix(". ")
        .or_else(|| after.strip_prefix(") "))?;
    Some((format!("{}.", &t[..digits]), rest.to_string()))
}

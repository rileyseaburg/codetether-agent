//! UTF-8-safe excerpts for bounded model-facing checkpoint rendering.
pub(super) fn excerpt(text: &str, budget: usize) -> String {
    if text.len() <= budget {
        return text.into();
    }
    let marker = "\n[excerpt; full text retained in checkpoint]\n";
    if budget < marker.len() {
        return String::new();
    }
    let keep = (budget - marker.len()) / 2;
    let mut head = keep;
    while !text.is_char_boundary(head) {
        head -= 1;
    }
    let mut tail = text.len() - keep;
    while !text.is_char_boundary(tail) {
        tail += 1;
    }
    format!("{}{marker}{}", &text[..head], &text[tail..])
}
pub(super) fn list(items: &[String], default: &str) -> String {
    if items.is_empty() {
        return format!("- {default}");
    }
    let mut text = String::new();
    if items.len() > 8 {
        text.push_str("[earlier entries retained in checkpoint]\n");
    }
    for item in items.iter().rev().take(8).rev() {
        text.push_str("- ");
        text.push_str(&excerpt(item, 240));
        text.push('\n');
    }
    text
}

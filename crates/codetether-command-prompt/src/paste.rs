//! Safe clipboard insertion for the single-line mux control prompt.

pub(super) fn single_line(text: &str) -> String {
    text.replace("\r\n", "\n")
        .replace('\r', "\n")
        .split('\n')
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::single_line;

    #[test]
    fn multiline_paste_cannot_submit_prompt_lines() {
        assert_eq!(single_line("first\r\nsecond\nthird"), "first second third");
    }

    #[test]
    fn ordinary_text_is_unchanged() {
        assert_eq!(single_line("mux command"), "mux command");
    }
}

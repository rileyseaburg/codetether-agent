//! FINAL extraction from LLM text responses.

/// Extract the answer from a `FINAL("…")` call in LLM text.
pub fn extract_final(text: &str) -> Option<String> {
    let start_idx = text.find("FINAL")?;
    let after = &text[start_idx..];
    let open_idx = after.find(['"', '\'', '`'])?;
    let quote_char = after[open_idx..].chars().next()?;
    let content_start = start_idx + open_idx + quote_char.len_utf8();
    let content = &text[content_start..];
    let close_idx = content.find(quote_char)?;
    let answer = &content[..close_idx];
    (!answer.is_empty()).then(|| answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_double_quoted() {
        assert_eq!(
            extract_final(r#"Some text FINAL("the answer") more"#),
            Some("the answer".into())
        );
    }

    #[test]
    fn extracts_single_quoted() {
        assert_eq!(
            extract_final("text FINAL('ans') x"),
            Some("ans".into())
        );
    }

    #[test]
    fn empty_answer_is_none() {
        assert_eq!(extract_final(r#"FINAL("")"#), None);
    }

    #[test]
    fn no_final_is_none() {
        assert_eq!(extract_final("just text"), None);
    }
}

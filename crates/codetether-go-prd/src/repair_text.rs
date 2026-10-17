//! Build JSON repair instructions, preserving the existing response excerpt limit.

pub(crate) fn build(last_text: &str, last_error: &str) -> String {
    format!(
        "Your previous response was not valid JSON. Here is the error:\n{err}\n\n\
                 Here is what you returned:\n```\n{text}\n```\n\n\
                 Please output ONLY the corrected JSON object — no markdown fences, \
                 no commentary, no trailing commas, no comments. Start with {{ and end with }}.",
        err = last_error,
        text = if last_text.len() > 2000 {
            &last_text[..2000]
        } else {
            last_text
        },
    )
}

//! Locate balanced top-level JSON objects inside free-form text.

/// Yield every balanced top-level `{...}` slice in `text`, in order.
///
/// String literals are respected so braces inside quoted text do not
/// disturb the depth count.
pub(super) fn objects(text: &str) -> impl DoubleEndedIterator<Item = &str> {
    let mut objects = Vec::new();
    let mut depth = 0usize;
    let mut start = None;
    let mut in_string = false;
    let mut escaped = false;
    for (index, ch) in text.char_indices() {
        if in_string {
            match ch {
                '\\' if !escaped => escaped = true,
                '"' if !escaped => in_string = false,
                _ => escaped = false,
            }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '{' => {
                if depth == 0 {
                    start = Some(index);
                }
                depth += 1;
            }
            '}' if depth > 0 => {
                depth -= 1;
                if depth == 0
                    && let Some(begin) = start.take()
                {
                    objects.push(&text[begin..=index]);
                }
            }
            _ => {}
        }
    }
    objects.into_iter()
}

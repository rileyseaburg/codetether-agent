//! Generated JavaScript brace matching.

pub fn matching(source: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (index, ch) in source[open..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(open + index);
                }
            }
            _ => {}
        }
    }
    None
}

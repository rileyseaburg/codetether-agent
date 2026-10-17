//! Preserve the initial request and recent activity text.

pub(super) fn head_tail(lines: &[&str]) -> (String, String) {
    let head: String = lines
        .iter()
        .take(30)
        .copied()
        .collect::<Vec<_>>()
        .join("\n");
    let tail: String = lines
        .iter()
        .rev()
        .take(80)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .copied()
        .collect::<Vec<_>>()
        .join("\n");

    (head, tail)
}

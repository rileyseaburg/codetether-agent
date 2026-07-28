//! Scanner for nested template literals.

pub(super) fn template(b: &[u8], start: usize) -> usize {
    let mut i = start + 1;
    while i < b.len() {
        match b[i] {
            b'\\' => i = (i + 2).min(b.len()),
            b'`' => return i + 1,
            b'$' if b.get(i + 1) == Some(&b'{') => {
                i = super::template_scan::expr_end(b, i + 2)
                    .saturating_add(1)
                    .min(b.len());
            }
            _ => i += 1,
        }
    }
    i
}

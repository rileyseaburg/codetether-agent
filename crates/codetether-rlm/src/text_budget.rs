//! Bounded chronological excerpts for transcript turns and summary sections.
//! Use [`pack_sections`] to divide space fairly and retain both ends of each part.

/// Divide a UTF-8 byte budget across ordered sections, redistributing unused space.
/// # Arguments
/// * `sections` — Ordered source parts, including their headings and delimiters.
/// * `budget` — Maximum output bytes, including omission markers.
/// # Returns
/// Complete short parts and marked head/tail excerpts of long parts. Extremely
/// small allocations omit the part; original source text is never changed.
/// # Examples
/// ```
/// use codetether_rlm::text_budget::pack_sections;
/// assert_eq!(pack_sections(&["first\n", "last"], 100), "first\nlast");
/// assert!(pack_sections(&[&"🦀".repeat(100)], 80).len() <= 80);
/// ```
pub fn pack_sections(sections: &[&str], budget: usize) -> String {
    let mut remaining = budget;
    let mut limits = vec![0; sections.len()];
    let mut pending: Vec<_> = (0..sections.len()).collect();
    while !pending.is_empty() && remaining > 0 {
        let share = (remaining / pending.len()).max(1);
        pending.retain(|&i| {
            let extra = share.min(sections[i].len() - limits[i]).min(remaining);
            limits[i] += extra;
            remaining -= extra;
            limits[i] < sections[i].len()
        });
    }
    sections
        .iter()
        .zip(limits)
        .map(|(text, limit)| excerpt(text, limit))
        .collect()
}

fn excerpt(text: &str, limit: usize) -> String {
    if text.len() <= limit {
        return text.into();
    }
    let marker = "\n[... excerpt omitted; consult original source ...]\n";
    if limit < marker.len() {
        return String::new();
    }
    let keep = (limit - marker.len()) / 2;
    let mut front = keep;
    let mut back = text.len() - keep;
    while !text.is_char_boundary(front) {
        front -= 1;
    }
    while !text.is_char_boundary(back) {
        back += 1;
    }
    format!("{}{marker}{}", &text[..front], &text[back..])
}

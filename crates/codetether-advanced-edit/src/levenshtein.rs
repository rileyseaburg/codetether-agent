//! levenshtein matching strategy.

/// Levenshtein distance for fuzzy matching.
///
/// Uses O(min(N,M)) space by keeping only the current and previous rows.
pub(super) fn levenshtein(a: &str, b: &str) -> usize {
    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (short, long) = if a.len() < b.len() {
        (&a, &b)
    } else {
        (&b, &a)
    };
    let mut prev_row: Vec<usize> = (0..=short.len()).collect();
    let mut curr_row = vec![0; short.len() + 1];
    for (i, &c_long) in long.iter().enumerate() {
        curr_row[0] = i + 1;
        for (j, &c_short) in short.iter().enumerate() {
            let cost = if c_long == c_short { 0 } else { 1 };
            curr_row[j + 1] = (curr_row[j] + 1)
                .min(prev_row[j + 1] + 1)
                .min(prev_row[j] + cost);
        }
        prev_row.copy_from_slice(&curr_row);
    }
    prev_row[short.len()]
}

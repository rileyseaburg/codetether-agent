//! Choose the single candidate or the most similar anchored block.
use super::levenshtein::levenshtein;
pub(super) fn select(
    orig_lines: &[&str],
    search_lines: &[&str],
    candidates: Vec<(usize, usize)>,
) -> Vec<String> {
    if candidates.len() == 1 {
        let (start, end) = candidates[0];
        return vec![orig_lines[start..=end].join("\n")];
    }
    // Multiple candidates: find best by similarity
    let mut best = None;
    let mut best_sim = -1.0f64;
    for (start, end) in candidates {
        let block_size = end - start + 1;
        let mut sim = 0.0;
        let lines_to_check = (search_lines.len() - 2).min(block_size - 2);
        if lines_to_check > 0 {
            for j in 1..search_lines.len().min(block_size) - 1 {
                let orig = orig_lines[start + j].trim();
                let search = search_lines[j].trim();
                let max_len = orig.len().max(search.len());
                if max_len > 0 {
                    let dist = levenshtein(orig, search);
                    sim += 1.0 - (dist as f64 / max_len as f64);
                }
            }
            sim /= lines_to_check as f64;
        } else {
            sim = 1.0;
        }
        if sim > best_sim {
            best_sim = sim;
            best = Some((start, end));
        }
    }
    if best_sim >= 0.3
        && let Some((s, e)) = best
    {
        return vec![orig_lines[s..=e].join("\n")];
    }
    vec![]
}

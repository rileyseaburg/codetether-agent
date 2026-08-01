//! Frequency-based selection of the authoritative answer candidate.
//!
//! Gemini streams cumulatively, so the real answer is repeated across frames
//! while drafts, titles, and leaked reasoning typically appear once. Observed
//! live: a 40 KB tool-result turn produced 15 frames where the final slot value
//! was unrelated content ("Call center agent... coffee shop") for a prompt about
//! neither. Taking the last candidate surfaced that as the answer.
//!
//! Selecting the most-repeated candidate instead makes a single stray frame
//! unable to displace a repeated answer.
//!
//! Short streams are a special case: a two-frame reply legitimately shows each
//! candidate once (draft then correction), so agreement cannot be required
//! there. Repetition is only used as a tie-breaker once enough frames exist for
//! it to be meaningful.

/// Frame count above which a repeated candidate is required to win.
///
/// Below this, order decides; the observed corruption needed 15 frames.
const CONSENSUS_MIN_CANDIDATES: usize = 4;

/// Chooses the answer from ordered candidates, or `None` when none repeats.
///
/// Ties are broken toward the later candidate, preserving the previous
/// last-wins behaviour for genuine cumulative replacements.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::gemini_web::response_text::consensus::select;
///
/// // Repeated answer wins over a single stray frame.
/// let frames = ["ok", "ok", "ok", "stray"].map(str::to_string);
/// let picked = select(&frames);
/// assert_eq!(picked.as_deref(), Some("ok"));
///
/// // Short stream: last wins, matching cumulative replacement.
/// let short = ["draft".to_string(), "final".to_string()];
/// assert_eq!(select(&short).as_deref(), Some("final"));
///
/// // A single candidate is unambiguous.
/// assert_eq!(select(&["only".to_string()]).as_deref(), Some("only"));
/// ```
pub fn select(candidates: &[String]) -> Option<String> {
    if candidates.is_empty() {
        return None;
    }
    if candidates.len() < CONSENSUS_MIN_CANDIDATES {
        return candidates.last().cloned();
    }

    let mut best: Option<(usize, usize, &String)> = None;
    for (index, candidate) in candidates.iter().enumerate() {
        let count = candidates
            .iter()
            .filter(|other| *other == candidate)
            .count();
        let better = best.is_none_or(|(top, at, _)| count > top || (count == top && index > at));
        if better {
            best = Some((count, index, candidate));
        }
    }

    // With many frames a repeated candidate is authoritative. If nothing
    // repeats the stream is unreliable, so fall back to order rather than
    // inventing agreement.
    match best {
        Some((count, _, value)) if count > 1 => Some(value.clone()),
        _ => candidates.last().cloned(),
    }
}

#[cfg(test)]
#[path = "consensus_tests.rs"]
mod tests;

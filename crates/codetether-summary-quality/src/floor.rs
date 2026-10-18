//! Minimum plausible summary size relative to input size.

/// Minimum plausible output-token count for an input of `input_tokens`.
///
/// Below ~256 input tokens any non-empty summary is acceptable. For
/// larger inputs we require the greater of 50 output tokens and
/// `input_tokens / 1000` (≈ 0.1 %). The ratio catches the production
/// failure mode (770 k → 180 tokens) that a flat-50 floor would miss.
pub(super) fn degraded_floor(input_tokens: usize) -> usize {
    if input_tokens < 256 {
        1
    } else {
        (input_tokens / 1000).max(50)
    }
}

//! Rejection of Gemini "card" placeholder frames.
//!
//! Gemini's web transport sometimes emits a trailing frame whose only content is
//! an internal rendering placeholder such as
//! `http://googleusercontent.com/card_content/0`. Because
//! [`super::latest`] intentionally keeps the *last* non-empty candidate (so a
//! model self-correction wins over an abandoned draft), such a frame would
//! otherwise replace the real answer and surface to the user as the entire
//! response.
//!
//! Observed live on `gemini-web-fast`, where a turn rendered as:
//! `http://googleusercontent.com/card_content/0` followed by a truncated
//! `I will`.

/// Host used by Gemini's internal card/placeholder URLs.
const CARD_HOST: &str = "googleusercontent.com/card_content";

/// Returns `true` when `text` carries no user-visible content.
///
/// A candidate is a placeholder when, after trimming, it consists solely of one
/// or more Gemini card URLs and whitespace.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::gemini_web::response_text::is_placeholder;
///
/// assert!(is_placeholder("http://googleusercontent.com/card_content/0"));
/// assert!(!is_placeholder("Here is the answer."));
/// assert!(!is_placeholder(
///     "See http://googleusercontent.com/card_content/0 for details."
/// ));
/// ```
pub fn is_placeholder(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() || !trimmed.contains(CARD_HOST) {
        return false;
    }
    trimmed
        .split_whitespace()
        .all(|token| token.contains(CARD_HOST))
}

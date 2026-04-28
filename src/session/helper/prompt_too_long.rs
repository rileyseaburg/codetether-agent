//! Retry plan for provider prompt-too-long errors.

use anyhow::Error;

use super::error::is_prompt_too_long_error;

const FORCE_KEEP_LAST_RETRIES: [usize; 3] = [6, 3, 1];

/// Return the forced keep-last value for this prompt-too-long attempt.
pub(crate) fn keep_last(error: &Error, attempt: usize) -> Option<usize> {
    if !is_prompt_too_long_error(error) {
        return None;
    }
    FORCE_KEEP_LAST_RETRIES
        .get(attempt.saturating_sub(1))
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cascades_to_single_latest_message() {
        let err = anyhow::anyhow!("Your input exceeds the context window");
        assert_eq!(keep_last(&err, 1), Some(6));
        assert_eq!(keep_last(&err, 2), Some(3));
        assert_eq!(keep_last(&err, 3), Some(1));
        assert_eq!(keep_last(&err, 4), None);
    }

    #[test]
    fn ignores_unrelated_errors() {
        let err = anyhow::anyhow!("connection reset");
        assert_eq!(keep_last(&err, 1), None);
    }
}

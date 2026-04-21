//! A tiny `(input, output)` token pair with a convenience `total()` method.

use serde::{Deserialize, Serialize};

/// Aggregate input / output token counts for a single scope
/// (a request, a model, or the whole process).
///
/// # Examples
///
/// ```rust
/// use codetether_agent::telemetry::TokenTotals;
///
/// let t = TokenTotals::new(1_200, 350);
/// assert_eq!(t.total(), 1_550);
///
/// // `Default` yields zero counts.
/// let zero = TokenTotals::default();
/// assert_eq!(zero.total(), 0);
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenTotals {
    /// Tokens consumed by the prompt / input side of a request.
    pub input: u64,
    /// Tokens produced in the completion / output side of a request.
    pub output: u64,
}

impl TokenTotals {
    /// Construct a new pair.
    pub fn new(input: u64, output: u64) -> Self {
        Self { input, output }
    }

    /// Sum of `input + output`.
    pub fn total(&self) -> u64 {
        self.input + self.output
    }
}

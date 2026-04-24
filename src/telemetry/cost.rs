//! USD cost estimation for LLM requests.

use serde::{Deserialize, Serialize};

use super::tokens::TokenCounts;

/// Estimated request cost, split into input/output components.
///
/// Prices are in **USD per million tokens** — the standard unit advertised by
/// Anthropic, OpenAI, and Bedrock.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::telemetry::{CostEstimate, TokenCounts};
///
/// let cost = CostEstimate::from_tokens(
///     &TokenCounts::new(2_000_000, 1_000_000),
///     3.00,
///     15.00,
/// );
/// assert!((cost.total_cost - 21.0).abs() < 1e-6);
/// assert_eq!(cost.format_smart(), "$21.00");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostEstimate {
    /// Cost attributable to input tokens.
    pub input_cost: f64,
    /// Cost attributable to output tokens.
    pub output_cost: f64,
    /// `input_cost + output_cost`.
    pub total_cost: f64,
    /// ISO-4217 currency code (always `"USD"` today).
    pub currency: String,
}

impl Default for CostEstimate {
    fn default() -> Self {
        Self {
            input_cost: 0.0,
            output_cost: 0.0,
            total_cost: 0.0,
            currency: "USD".to_string(),
        }
    }
}

impl CostEstimate {
    /// Compute a cost estimate from token counts and per-million prices.
    pub fn from_tokens(tokens: &TokenCounts, input_price: f64, output_price: f64) -> Self {
        let input_cost = (tokens.input_tokens as f64 / 1_000_000.0) * input_price;
        let output_cost = (tokens.output_tokens as f64 / 1_000_000.0) * output_price;
        Self {
            input_cost,
            output_cost,
            total_cost: input_cost + output_cost,
            currency: "USD".to_string(),
        }
    }

    /// Always 4 decimal places: `"$0.0042"`.
    pub fn format_currency(&self) -> String {
        format!("${:.4}", self.total_cost)
    }

    /// Sub-cent amounts use 4 decimals; `>= $0.01` uses 2.
    pub fn format_smart(&self) -> String {
        if self.total_cost < 0.01 {
            format!("${:.4}", self.total_cost)
        } else {
            format!("${:.2}", self.total_cost)
        }
    }
}

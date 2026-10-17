//! Token cost estimation using supplied per-million pricing.

/// Pricing from models.dev for a single model
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// let price = codetether_benchmark::pricing::ModelPricing { input_per_m: 1.0, output_per_m: 2.0 };
/// assert_eq!(price.input_per_m, 1.0);
/// ```
pub struct ModelPricing {
    /// Cost per million input tokens (USD)
    pub input_per_m: f64,
    /// Cost per million output tokens (USD)
    pub output_per_m: f64,
}

/// Calculate cost from token counts and pricing
///
/// # Arguments
/// * `input_tokens` - Consumed input tokens.
/// * `output_tokens` - Consumed output tokens.
/// * `pricing` - Optional supplied per-million prices.
/// # Returns
/// Estimated USD cost, retaining the original fallback when prices are absent.
/// # Examples
/// ```
/// assert_eq!(codetether_benchmark::pricing::calculate_cost(0, 0, &None), 0.0);
/// ```
pub fn calculate_cost(
    input_tokens: u64,
    output_tokens: u64,
    pricing: &Option<ModelPricing>,
) -> f64 {
    match pricing {
        Some(p) => {
            (input_tokens as f64 / 1_000_000.0) * p.input_per_m
                + (output_tokens as f64 / 1_000_000.0) * p.output_per_m
        }
        // Fallback: ~$5/1M tokens average when pricing unavailable
        None => (input_tokens + output_tokens) as f64 * 0.000005,
    }
}

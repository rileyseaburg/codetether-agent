//! Existing premium-request cost approximations.
use crate::*;
impl CopilotProvider {
    /// Enrich models with pricing metadata from known premium request multipliers.
    ///
    /// Source: https://docs.github.com/en/copilot/concepts/billing/copilot-requests
    /// Cost model: Premium requests at $0.04/request overflow rate.
    /// We convert multiplier to approximate $/M tokens using ~4K tokens/request avg.
    /// Formula: multiplier * $0.04 / 4K tokens * 1M = multiplier * $10/M tokens.
    pub(crate) fn enrich_with_pricing(&self, models: &mut [ModelInfo]) {
        // (display_name, premium_multiplier)
        let pricing: std::collections::HashMap<&str, (&str, f64)> = rows::all().copied().collect();

        for model in models.iter_mut() {
            if let Some((display_name, premium_mult)) = pricing.get(model.id.as_str()) {
                // Set a friendlier display name when the API only returned the raw id
                if model.name == model.id {
                    model.name = display_name.to_string();
                }
                let approx_cost = premium_mult * 10.0;
                model.input_cost_per_million = Some(approx_cost);
                model.output_cost_per_million = Some(approx_cost);
            } else {
                // Unknown Copilot model — assume 1x premium request ($10/M approx)
                if model.input_cost_per_million.is_none() {
                    model.input_cost_per_million = Some(10.0);
                }
                if model.output_cost_per_million.is_none() {
                    model.output_cost_per_million = Some(10.0);
                }
            }
        }
    }
}
mod rows;

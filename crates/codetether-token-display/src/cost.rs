//! Model pricing and accumulated session cost.
use codetether_telemetry::{CostEstimate, TOKEN_USAGE};
impl crate::TokenDisplay {
    /// Get pricing for a model (returns $ per million tokens for input/output).
    ///
    /// Delegates to the canonical
    /// [`codetether_provider_core::pricing::pricing_for_model`] so costs in the TUI
    /// stay in sync with the cost-guardrail enforcement path.
    pub(crate) fn get_model_pricing(&self, model: &str) -> (f64, f64) {
        codetether_provider_core::pricing::pricing_for_model(model)
    }

    /// Calculate cost for a model given input and output token counts
    /// Estimate cost using shared model prices.
    /// # Arguments
    /// * `model` - Model pricing identifier.
    /// * `input_tokens` - Input token count.
    /// * `output_tokens` - Output token count.
    /// # Returns
    /// Input, output and total cost estimates.
    /// # Examples
    /// ```
    /// let _cost = codetether_token_display::TokenDisplay::new().calculate_cost_for_tokens("model", 0, 0);
    /// ```
    pub fn calculate_cost_for_tokens(
        &self,
        model: &str,
        input_tokens: u64,
        output_tokens: u64,
    ) -> CostEstimate {
        let (input_price, output_price) = self.get_model_pricing(model);
        CostEstimate::from_tokens(
            &codetether_telemetry::TokenCounts::new(input_tokens, output_tokens),
            input_price,
            output_price,
        )
    }

    /// Calculate total session cost across all models
    /// Aggregate cost across recorded models.
    /// # Arguments
    /// None.
    /// # Returns
    /// The sum of per-model input, output and total estimates.
    /// # Examples
    /// ```
    /// let _cost = codetether_token_display::TokenDisplay::new().calculate_session_cost();
    /// ```
    pub fn calculate_session_cost(&self) -> CostEstimate {
        let model_snapshots = TOKEN_USAGE.model_snapshots();
        let mut total = CostEstimate::default();

        for snapshot in model_snapshots {
            let model_cost = self.calculate_cost_for_tokens(
                &snapshot.name,
                snapshot.totals.input,
                snapshot.totals.output,
            );
            total.input_cost += model_cost.input_cost;
            total.output_cost += model_cost.output_cost;
            total.total_cost += model_cost.total_cost;
        }

        total
    }
}

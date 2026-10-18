//! Detailed usage totals, model rows, provider metrics and cost.
use codetether_telemetry::TOKEN_USAGE;
impl crate::TokenDisplay {
    /// Create detailed token usage display
    /// Build detailed usage, provider and cost text.
    /// # Arguments
    /// None.
    /// # Returns
    /// Formatted usage lines in display order.
    /// # Examples
    /// ```
    /// let lines = codetether_token_display::TokenDisplay::new().create_detailed_display(); assert!(!lines.is_empty());
    /// ```
    pub fn create_detailed_display(&self) -> Vec<String> {
        let mut lines = Vec::new();
        let global_snapshot = TOKEN_USAGE.global_snapshot();
        let model_snapshots = TOKEN_USAGE.model_snapshots();

        lines.push("".to_string());
        lines.push("  TOKEN USAGE & COSTS".to_string());
        lines.push("  ===================".to_string());
        lines.push("".to_string());

        // Global totals
        let total_cost = self.calculate_session_cost();
        lines.push(format!(
            "  Total: {} tokens ({} requests) - {}",
            global_snapshot.totals.total(),
            global_snapshot.request_count,
            total_cost.format_currency()
        ));
        lines.push(format!(
            "  Current: {} in / {} out",
            global_snapshot.totals.input, global_snapshot.totals.output
        ));
        lines.push("".to_string());

        crate::models::append(&mut lines, self, &model_snapshots);
        crate::providers::append(&mut lines);
        // Cost estimates
        lines.push("  COST ESTIMATES:".to_string());
        lines.push(format!(
            "    Session total: {}",
            total_cost.format_currency()
        ));
        lines.push("    Based on approximate pricing".to_string());

        lines
    }
}

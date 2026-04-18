use crate::telemetry::{ContextLimit, CostEstimate, TOKEN_USAGE, TokenUsageSnapshot};
use crate::tui::theme::Theme;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Enhanced token usage display with costs and warnings
pub struct TokenDisplay {
    /// Pricing data (not context limits — those come from the canonical
    /// [`crate::provider::limits::context_window_for_model`]).
    model_pricing: std::collections::HashMap<String, (f64, f64)>,
}

impl TokenDisplay {
    pub fn new() -> Self {
        Self {
            model_pricing: std::collections::HashMap::new(),
        }
    }

    /// Get context limit for a model by delegating to the canonical
    /// [`crate::provider::limits::context_window_for_model`].
    ///
    /// Returns `None` only for the zero-length model string edge case;
    /// callers that relied on the old `HashMap::get` returning `None`
    /// for unknown models will now get the default 128 000 instead,
    /// which is strictly more correct (all models have *some* context
    /// window).
    pub fn get_context_limit(&self, model: &str) -> Option<u64> {
        if model.is_empty() {
            return None;
        }
        Some(crate::provider::limits::context_window_for_model(model) as u64)
    }

    /// Get pricing for a model (returns $ per million tokens for input/output)
    fn get_model_pricing(&self, model: &str) -> (f64, f64) {
        match model.to_lowercase().as_str() {
            m if m.contains("gpt-4o-mini") => (0.15, 0.60), // $0.15 / $0.60 per million
            m if m.contains("gpt-4o") => (2.50, 10.00),     // $2.50 / $10.00 per million
            m if m.contains("gpt-4-turbo") => (10.00, 30.00), // $10 / $30 per million
            m if m.contains("gpt-4") => (30.00, 60.00),     // $30 / $60 per million
            m if m.contains("claude-3-5-sonnet") => (3.00, 15.00), // $3 / $15 per million
            m if m.contains("claude-3-5-haiku") => (0.80, 4.00), // $0.80 / $4 per million
            m if m.contains("claude-opus") => (5.00, 25.00), // $5 / $25 per million (Bedrock Opus 4.6)
            m if m.contains("gemini-2.0-flash") => (0.075, 0.30), // $0.075 / $0.30 per million
            m if m.contains("gemini-1.5-flash") => (0.075, 0.30), // $0.075 / $0.30 per million
            m if m.contains("gemini-1.5-pro") => (1.25, 5.00), // $1.25 / $5 per million
            m if m.contains("glm-4") => (0.50, 0.50),        // ZhipuAI GLM-4 ~$0.50/million
            m if m.contains("k1.5") => (8.00, 8.00),         // Moonshot K1.5
            m if m.contains("k1.6") => (6.00, 6.00),         // Moonshot K1.6
            _ => (1.00, 3.00),                               // Default fallback
        }
    }

    /// Calculate cost for a model given input and output token counts
    pub fn calculate_cost_for_tokens(
        &self,
        model: &str,
        input_tokens: u64,
        output_tokens: u64,
    ) -> CostEstimate {
        let (input_price, output_price) = self.get_model_pricing(model);
        CostEstimate::from_tokens(
            &crate::telemetry::TokenCounts::new(input_tokens, output_tokens),
            input_price,
            output_price,
        )
    }

    /// Create status bar content with token usage
    pub fn create_status_bar(&self, theme: &Theme) -> Line<'_> {
        let global_snapshot = TOKEN_USAGE.global_snapshot();
        let model_snapshots = TOKEN_USAGE.model_snapshots();

        let total_tokens = global_snapshot.totals.total();
        let session_cost = self.calculate_session_cost();
        let tps_display = self.get_tps_display();

        let mut spans = Vec::new();

        // Help indicator
        spans.push(Span::styled(
            " ? ",
            Style::default()
                .fg(theme.status_bar_foreground.to_color())
                .bg(theme.status_bar_background.to_color()),
        ));
        spans.push(Span::raw(" Help "));

        // Switch agent
        spans.push(Span::styled(
            " Tab ",
            Style::default()
                .fg(theme.status_bar_foreground.to_color())
                .bg(theme.status_bar_background.to_color()),
        ));
        spans.push(Span::raw(" Switch Agent "));

        // Quit
        spans.push(Span::styled(
            " Ctrl+C ",
            Style::default()
                .fg(theme.status_bar_foreground.to_color())
                .bg(theme.status_bar_background.to_color()),
        ));
        spans.push(Span::raw(" Quit "));

        // Token usage
        spans.push(Span::styled(
            format!(" Tokens: {} ", total_tokens),
            Style::default().fg(theme.timestamp_color.to_color()),
        ));

        // TPS (tokens per second)
        if let Some(tps) = tps_display {
            spans.push(Span::styled(
                format!(" TPS: {} ", tps),
                Style::default().fg(Color::Cyan),
            ));
        }

        // Cost
        spans.push(Span::styled(
            format!(" Cost: {} ", session_cost.format_smart()),
            Style::default().fg(theme.timestamp_color.to_color()),
        ));

        // Context warning if active model is near limit
        if let Some(warning) = self.get_context_warning(&model_snapshots) {
            spans.push(Span::styled(
                format!(" {} ", warning),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ));
        }

        Line::from(spans)
    }

    /// Calculate total session cost across all models
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

    /// Get context warning for active model
    fn get_context_warning(&self, model_snapshots: &[TokenUsageSnapshot]) -> Option<String> {
        if model_snapshots.is_empty() {
            return None;
        }

        // Use the model with highest usage as "active"
        let active_model = model_snapshots.iter().max_by_key(|s| s.totals.total())?;

        if let Some(limit) = self.get_context_limit(&active_model.name) {
            let context = ContextLimit::new(active_model.totals.total(), limit);

            if context.percentage >= 75.0 {
                return Some(format!("⚠️ Context: {:.1}%", context.percentage));
            }
        }

        None
    }

    /// Get TPS (tokens per second) display string from provider metrics
    fn get_tps_display(&self) -> Option<String> {
        use crate::telemetry::PROVIDER_METRICS;

        let snapshots = PROVIDER_METRICS.all_snapshots();
        if snapshots.is_empty() {
            return None;
        }

        // Find the provider with the most recent activity
        let most_active = snapshots
            .iter()
            .filter(|s| s.avg_tps > 0.0)
            .max_by(|a, b| {
                a.total_output_tokens
                    .partial_cmp(&b.total_output_tokens)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })?;

        // Format TPS nicely
        let tps = most_active.avg_tps;
        let formatted = if tps >= 100.0 {
            format!("{:.0}", tps)
        } else if tps >= 10.0 {
            format!("{:.1}", tps)
        } else {
            format!("{:.2}", tps)
        };

        Some(formatted)
    }

    /// Create detailed token usage display
    pub fn create_detailed_display(&self) -> Vec<String> {
        use crate::telemetry::PROVIDER_METRICS;

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

        // Per-model breakdown
        if !model_snapshots.is_empty() {
            lines.push("  BY MODEL:".to_string());

            for snapshot in model_snapshots.iter().take(5) {
                let model_cost = self.calculate_cost_for_tokens(
                    &snapshot.name,
                    snapshot.totals.input,
                    snapshot.totals.output,
                );
                lines.push(format!(
                    "    {}: {} tokens ({} requests) - {}",
                    snapshot.name,
                    snapshot.totals.total(),
                    snapshot.request_count,
                    model_cost.format_currency()
                ));

                // Context limit info
                if let Some(limit) = self.get_context_limit(&snapshot.name) {
                    let context = ContextLimit::new(snapshot.totals.total(), limit);
                    if context.percentage >= 50.0 {
                        lines.push(format!(
                            "      Context: {:.1}% of {} tokens",
                            context.percentage, limit
                        ));
                    }
                }
            }

            if model_snapshots.len() > 5 {
                lines.push(format!(
                    "    ... and {} more models",
                    model_snapshots.len() - 5
                ));
            }
            lines.push("".to_string());
        }

        // Provider performance metrics (TPS, latency)
        let provider_snapshots = PROVIDER_METRICS.all_snapshots();
        if !provider_snapshots.is_empty() {
            lines.push("  PROVIDER PERFORMANCE:".to_string());

            for snapshot in provider_snapshots.iter().take(5) {
                if snapshot.request_count > 0 {
                    lines.push(format!(
                        "    {}: {:.1} avg TPS | {:.0}ms avg latency | {} reqs",
                        snapshot.provider,
                        snapshot.avg_tps,
                        snapshot.avg_latency_ms,
                        snapshot.request_count
                    ));

                    // Show p50/p95 if we have enough requests
                    if snapshot.request_count >= 5 {
                        lines.push(format!(
                            "      p50: {:.1} TPS / {:.0}ms | p95: {:.1} TPS / {:.0}ms",
                            snapshot.p50_tps,
                            snapshot.p50_latency_ms,
                            snapshot.p95_tps,
                            snapshot.p95_latency_ms
                        ));
                    }
                }
            }
            lines.push("".to_string());
        }

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

impl Default for TokenDisplay {
    fn default() -> Self {
        Self::new()
    }
}

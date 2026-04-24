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

    /// Get pricing for a model (returns $ per million tokens for input/output).
    ///
    /// Delegates to the canonical
    /// [`crate::provider::pricing::pricing_for_model`] so costs in the TUI
    /// stay in sync with the cost-guardrail enforcement path.
    fn get_model_pricing(&self, model: &str) -> (f64, f64) {
        if let Some(pricing) = self.model_pricing.get(model) {
            return *pricing;
        }
        crate::provider::pricing::pricing_for_model(model)
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

        // Cost — colorized based on the configured cost guardrails so
        // users get an immediate visual when they cross warn / hard limit
        // thresholds (see `CODETETHER_COST_WARN_USD` /
        // `CODETETHER_COST_LIMIT_USD`).
        let cost_style = match crate::session::helper::cost_guard::cost_guard_level() {
            crate::session::helper::cost_guard::CostGuardLevel::OverLimit => {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            }
            crate::session::helper::cost_guard::CostGuardLevel::OverWarn => Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            crate::session::helper::cost_guard::CostGuardLevel::Ok => {
                Style::default().fg(theme.timestamp_color.to_color())
            }
        };
        spans.push(Span::styled(
            format!(" Cost: {} ", session_cost.format_smart()),
            cost_style,
        ));

        // Prompt-cache hit rate — surfaces whether Anthropic/Bedrock
        // prompt caching is actually saving input tokens. Only shown when
        // the active model has recorded any cache activity at all.
        if let Some(cache_pct) = self.get_cache_hit_rate(&model_snapshots) {
            spans.push(Span::styled(
                format!(" Cache: {:.0}% ", cache_pct),
                Style::default().fg(Color::Green),
            ));
        }

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

    /// Get context warning for the active model based on the **current
    /// turn's** prompt size (what the next request will send), not
    /// cumulative lifetime tokens.
    ///
    /// This matters because the agent loop re-sends the growing
    /// conversation on every step and the RLM layer compresses history
    /// behind the scenes — users need to see the real edge of the
    /// context window, not a bogus 6000% figure summed over all turns.
    /// This matters because the agent loop re-sends the growing
    /// conversation on every step and the RLM layer compresses history
    /// behind the scenes — users need to see the real edge of the
    /// context window, not a bogus 6000% figure summed over all turns.
    fn get_context_warning(&self, model_snapshots: &[TokenUsageSnapshot]) -> Option<String> {
        if model_snapshots.is_empty() {
            return None;
        }

        let active_model = model_snapshots.iter().max_by_key(|s| s.totals.total())?;
        let limit = self.get_context_limit(&active_model.name)?;

        // Prefer the last turn's actual prompt size; fall back to cumulative
        // only if no turn has been recorded yet (first-render race).
        let used = crate::telemetry::TOKEN_USAGE
            .last_prompt_tokens_for(&active_model.name)
            .unwrap_or_else(|| active_model.totals.total().min(limit));

        let context = ContextLimit::new(used, limit);

        if context.percentage >= 90.0 {
            Some(format!("🛑 Context: {:.0}%", context.percentage))
        } else if context.percentage >= 75.0 {
            Some(format!("⚠️ Context: {:.0}%", context.percentage))
        } else if context.percentage >= 50.0 {
            Some(format!("Context: {:.0}%", context.percentage))
        } else {
            None
        }
    }

    /// Aggregate prompt-cache hit rate across all recorded models.
    ///
    /// Defined as `cache_read / (cache_read + full_price_input)` × 100,
    /// i.e. what fraction of billable input was served from the cache.
    /// Returns `None` when no cache activity has been recorded (which is
    /// the common case for providers that don't support it).
    fn get_cache_hit_rate(&self, model_snapshots: &[TokenUsageSnapshot]) -> Option<f64> {
        let mut full_input: u64 = 0;
        let mut cache_read: u64 = 0;
        for s in model_snapshots {
            full_input += s.prompt_tokens;
            let (cr, _cw) = crate::telemetry::TOKEN_USAGE.cache_usage_for(&s.name);
            cache_read += cr;
        }
        let denom = full_input + cache_read;
        if cache_read == 0 || denom == 0 {
            return None;
        }
        Some(cache_read as f64 * 100.0 / denom as f64)
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

                // Prompt-cache stats (Anthropic / Bedrock).
                let (cache_read, cache_write) =
                    crate::telemetry::TOKEN_USAGE.cache_usage_for(&snapshot.name);
                if cache_read > 0 || cache_write > 0 {
                    let denom = snapshot.prompt_tokens + cache_read;
                    let hit_pct = if denom > 0 {
                        cache_read as f64 * 100.0 / denom as f64
                    } else {
                        0.0
                    };
                    lines.push(format!(
                        "      Cache: {} read / {} write ({:.1}% hit)",
                        cache_read, cache_write, hit_pct
                    ));
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

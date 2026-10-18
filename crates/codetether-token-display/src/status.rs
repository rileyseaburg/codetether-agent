//! Status-bar assembly using live telemetry.
use codetether_telemetry::TOKEN_USAGE;
use codetether_theme::Theme;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
impl crate::TokenDisplay {
    /// Create status bar content with token usage
    /// Build a status bar from current telemetry.
    /// # Arguments
    /// * `theme` - Display colors.
    /// # Returns
    /// A styled status line with navigation, tokens, throughput, cost and context.
    /// # Examples
    /// ```
    /// let display = codetether_token_display::TokenDisplay::new(); let _line = display.create_status_bar(&codetether_theme::Theme::dark());
    /// ```
    pub fn create_status_bar(&self, theme: &Theme) -> Line<'_> {
        let global_snapshot = TOKEN_USAGE.global_snapshot();
        let model_snapshots = TOKEN_USAGE.model_snapshots();

        let total_tokens = global_snapshot.totals.total();
        let session_cost = self.calculate_session_cost();
        let tps_display = self.get_tps_display();

        let mut spans = Vec::new();

        crate::navigation::append(&mut spans, theme);
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

        crate::cost_span::append(&mut spans, theme, session_cost);
        // Prompt-cache hit rate — surfaces whether Anthropic/Bedrock
        // prompt caching is actually saving input tokens. Only shown when
        // the active model has recorded any cache activity at all.
        if let Some(cache_pct) = self.get_cache_hit_rate(&model_snapshots) {
            spans.push(Span::styled(
                format!(" Cache: {:.0}% ", cache_pct),
                Style::default().fg(Color::Green),
            ));
        }

        // Context usage — always visible so the user can see how full the
        // window is at a glance, with color escalating as it fills up.
        if let Some((label, style)) = self.get_context_indicator(&model_snapshots) {
            spans.push(Span::styled(format!(" {label} "), style));
        }

        Line::from(spans)
    }
}

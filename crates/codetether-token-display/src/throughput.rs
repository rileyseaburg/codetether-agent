//! Format throughput from the most active provider.

impl crate::TokenDisplay {
    /// Get TPS (tokens per second) display string from provider metrics
    pub(crate) fn get_tps_display(&self) -> Option<String> {
        use codetether_telemetry::PROVIDER_METRICS;

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
}

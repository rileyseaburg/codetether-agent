//! Prompt-cache counters and hit percentage for one model.
use codetether_telemetry::TokenUsageSnapshot;
pub(crate) fn append(lines: &mut Vec<String>, snapshot: &TokenUsageSnapshot) {
    // Prompt-cache stats (Anthropic / Bedrock).
    let (cache_read, cache_write) =
        codetether_telemetry::TOKEN_USAGE.cache_usage_for(&snapshot.name);
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

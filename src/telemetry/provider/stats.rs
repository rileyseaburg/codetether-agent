//! Pure helpers used by [`super::ProviderMetrics::all_snapshots`].
//!
//! Kept in their own file so the orchestration code in `metrics.rs` stays
//! small and readable.

/// Return the value at the `(n * pct) as usize` index of `sorted_values`,
/// or `0.0` if out of bounds. Inputs must be pre-sorted ascending.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::telemetry::provider::stats::percentile;
///
/// let xs = [1.0, 2.0, 3.0, 4.0, 5.0];
/// assert_eq!(percentile(&xs, 0.50), 3.0);
/// assert_eq!(percentile(&xs, 0.95), 5.0);
/// assert_eq!(percentile(&[], 0.50), 0.0);
/// ```
pub fn percentile(sorted_values: &[f64], pct: f64) -> f64 {
    if sorted_values.is_empty() {
        return 0.0;
    }
    let idx = (sorted_values.len() as f64 * pct) as usize;
    sorted_values.get(idx).copied().unwrap_or(0.0)
}

/// Sort a `Vec<f64>` ascending, treating NaN as equal. Pure utility so the
/// unwrap-on-ordering pattern only appears here.
pub fn sort_f64(values: &mut [f64]) {
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
}

/// Arithmetic mean of `values`. Returns `0.0` for empty input.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::telemetry::provider::stats::mean;
///
/// assert_eq!(mean(&[2.0, 4.0, 6.0]), 4.0);
/// assert_eq!(mean(&[]), 0.0);
/// ```
pub fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

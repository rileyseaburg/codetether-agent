//! Apply the final input-budget clamp and collect derivation provenance.
use super::*;
/// Finalize selection against a prepared summary index.
///
/// # Arguments
/// * `prepared` - Plan created before summary-index acquisition.
/// * `index` - Prepared summary snapshot supplied by the host.
/// * `estimate_request` - Estimate including fixed request overhead.
/// * `estimate_messages` - Estimate used to charge rendered summaries.
///
/// # Returns
/// Paired, budget-limited context with source coverage and provenance.
///
/// # Examples
/// See the end-to-end example on [PreparedIncremental].
pub fn finish(
    prepared: PreparedIncremental,
    index: &SummaryIndex,
    estimate_request: impl Fn(&[Message]) -> usize,
    estimate_messages: impl Fn(&[Message]) -> usize,
) -> DerivedContext {
    let mut validated = index.clone();
    validated.retain_valid(&prepared.clone);
    let mut plan = assemble::assemble(prepared, &validated, estimate_messages);
    let mut provenance = vec!["incremental".to_string()];
    let (final_dropped, tags) = clamp_and_recompute(
        &mut plan.messages,
        &mut plan.resolutions,
        &mut plan.origins,
        plan.budget_tokens,
        plan.clone_len,
        &plan.dropped_ranges,
        estimate_request,
    );
    provenance.extend(tags.into_iter().map(str::to_string));
    DerivedContext {
        resolutions: plan.resolutions,
        dropped_ranges: final_dropped,
        provenance,
        messages: plan.messages,
        origin_len: plan.origin_len,
        compressed: true,
    }
}

//! Prepare a selection, acquire fresh summaries and finish the projection.
use anyhow::Result;
use codetether_context_policy::{
    derivation::incremental::{finish, prepare},
    projection::helpers::DerivedContext,
};
use codetether_provider_core::ToolDefinition;
use codetether_request_tokens::{estimate_request_tokens, estimate_tokens_for_messages};
/// Derive a budgeted projection using fresh prepared summaries.
/// # Arguments
/// * `session` — Immutable messages, page classification and index access.
/// * `system_prompt` — System text included in the request estimate.
/// * `tools` — Tool definitions included in the estimate.
/// * `budget_tokens` — Available input-token budget.
/// # Returns
/// A repaired projection with residency, origin and compression metadata.
/// Compacted requests reserve room for bounded excerpts of the earliest loaded
/// human request and recent corrections, independent of generated summaries.
/// # Errors
/// Retains the fallible derivation contract for caller integration.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_incremental_derivation::Source>(session: &H) -> anyhow::Result<()> {
/// let projection = codetether_incremental_derivation::derive_incremental(session, "", &[], 128_000).await?;
/// assert_eq!(projection.origin_len, session.messages().len());
/// # Ok(()) }
/// ```
pub async fn derive_incremental<H: crate::Source>(
    session: &H,
    system_prompt: &str,
    tools: &[ToolDefinition],
    budget_tokens: usize,
) -> Result<DerivedContext> {
    let origin_len = session.messages().len();
    let mut clone = session.messages().to_vec();
    if let Some(ctx) =
        crate::try_pass_through(&mut clone, system_prompt, tools, budget_tokens, origin_len)
    {
        return Ok(ctx);
    }

    let message_budget =
        budget_tokens.saturating_sub(estimate_request_tokens(system_prompt, &[], tools));
    let references =
        super::references::References::new(&clone, message_budget, session.source_offset());
    let reference_cost = references.cost();
    let estimate = |messages: &[_]| {
        estimate_request_tokens(system_prompt, messages, tools).saturating_add(reference_cost)
    };

    let selection = prepare(
        clone,
        origin_len,
        budget_tokens,
        estimate,
        estimate_tokens_for_messages,
        |keep, costs, recent, budget| {
            super::required::seed_source(session, keep, costs, recent, budget)
        },
    );
    let prepared = super::prepared::index(session).await;
    let mut derived = finish(selection, &prepared, estimate, estimate_tokens_for_messages);
    if estimate(&derived.messages) <= budget_tokens {
        references.prepend(&mut derived);
    }
    Ok(derived)
}

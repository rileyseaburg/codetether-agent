//! [`DerivePolicy::Reset`] — Lu et al. reset-to-(prompt, summary) semantic.

use std::sync::Arc;

use anyhow::Result;

use crate::provider::ToolDefinition;
use crate::session::ResidencyLevel;
use crate::session::Session;
use crate::session::helper::experimental;
use crate::session::helper::token::estimate_request_tokens;

use super::derive::derive_context;
use super::helpers::DerivedContext;
use super::reset_rebuild::rebuild_with_summary;
use super::{active_tail::active_user_tail_start, reset_helpers::latest_reset_marker_index};

/// [`DerivePolicy::Reset`](crate::session::derive_policy::DerivePolicy::Reset) implementation.
///
/// When the token estimate exceeds `threshold_tokens`, summarise
/// everything older than the active task tail via the RLM router and
/// return `[summary_message, active_task_tail...]`. The tail starts at
/// the latest substantive user turn, so short follow-ups like "continue"
/// and "status?" do not orphan the real task request. Under threshold,
/// return the full clone verbatim.
pub(super) async fn derive_reset(
    session: &Session,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    threshold_tokens: usize,
) -> Result<DerivedContext> {
    let origin_len = session.messages.len();
    let mut messages = session.messages.clone();
    let mut dropped_ranges = Vec::new();
    let mut provenance = Vec::new();
    let mut base_index = 0usize;

    let est = estimate_request_tokens(system_prompt, &messages, tools);
    if est <= threshold_tokens {
        experimental::pairing::repair_orphans(&mut messages);
        return Ok(DerivedContext {
            resolutions: vec![ResidencyLevel::Full; messages.len()],
            dropped_ranges: Vec::new(),
            provenance: vec!["reset_below_threshold".to_string()],
            messages,
            origin_len,
            compressed: false,
        });
    }

    if let Some(reset_idx) = latest_reset_marker_index(&messages).filter(|idx| *idx > 0) {
        messages = messages.split_off(reset_idx);
        base_index = reset_idx;
        dropped_ranges.push((0, reset_idx));
        provenance.push("reset_marker".to_string());

        let anchored_est = estimate_request_tokens(system_prompt, &messages, tools);
        if anchored_est <= threshold_tokens {
            experimental::pairing::repair_orphans(&mut messages);
            return Ok(DerivedContext {
                resolutions: vec![ResidencyLevel::Full; messages.len()],
                dropped_ranges,
                provenance,
                messages,
                origin_len,
                compressed: true,
            });
        }
    }

    let Some(split_idx) = active_user_tail_start(&messages, 8) else {
        if !provenance.is_empty() {
            experimental::pairing::repair_orphans(&mut messages);
            return Ok(DerivedContext {
                resolutions: vec![ResidencyLevel::Full; messages.len()],
                dropped_ranges,
                provenance,
                messages,
                origin_len,
                compressed: true,
            });
        }
        return derive_context(session, provider, model, system_prompt, tools, None, None).await;
    };

    let tail = messages.split_off(split_idx);
    let prefix = std::mem::take(&mut messages);
    if prefix.is_empty() {
        messages = tail;
        experimental::pairing::repair_orphans(&mut messages);
        return Ok(DerivedContext {
            resolutions: vec![ResidencyLevel::Full; messages.len()],
            dropped_ranges,
            provenance: if provenance.is_empty() {
                vec!["reset_without_prefix".to_string()]
            } else {
                provenance
            },
            messages,
            origin_len,
            compressed: base_index > 0,
        });
    }

    let prefix_len = prefix.len();
    let mut derived =
        rebuild_with_summary(session, &prefix, &tail, provider, model, origin_len).await?;
    dropped_ranges.push((base_index, base_index + prefix_len));
    if !provenance.is_empty() {
        provenance.extend(derived.provenance);
        derived.provenance = provenance;
    }
    derived.dropped_ranges = dropped_ranges;
    Ok(derived)
}

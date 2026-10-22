//! Regenerate recent cached ranges through the actual repaired RLM engine.
use super::replay_support::Session;
use anyhow::Result;
use codetether_agent::provider::Provider;
use codetether_context_policy::summary_index::{
    Granularity, SummaryNode, SummaryProvenance, SummaryRange,
};
use std::sync::Arc;
#[path = "synthesize.rs"]
mod synthesize;
pub(super) async fn recent(
    source: &mut Session,
    provider: Arc<dyn Provider>,
    model: &str,
    report: &str,
) -> Result<()> {
    let end = source.messages.len();
    let mut notes = Vec::new();
    let closed = (source.offset + end).saturating_sub(8) / 16 * 16;
    for global in (closed.saturating_sub(48)..closed).step_by(16) {
        let start = global
            .checked_sub(source.offset)
            .expect("loaded replay range");
        let range = SummaryRange::new(start, start + 16).unwrap();
        let slice = &source.messages[range.start..range.end];
        let content = synthesize::summary(slice, provider.clone(), model).await?;
        notes.push(serde_json::json!({"start":source.offset+start,"end":source.offset+range.end,"summary":content}));
        source.background.insert(
            range,
            SummaryNode {
                content,
                target_tokens: 512,
                granularity: Granularity::Phase,
                generation: 1,
                provenance: Some(SummaryProvenance::new(
                    slice,
                    "live-replay-engine",
                    Some(model),
                )?),
            },
        );
    }
    std::fs::write(
        format!("{report}.summaries-{}.json", source.offset + end),
        serde_json::to_string_pretty(&notes)?,
    )?;
    Ok(())
}

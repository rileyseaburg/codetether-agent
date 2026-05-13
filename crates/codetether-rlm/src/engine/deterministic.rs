//! Deterministic answer routing.

use anyhow::Result;

use crate::router::CrateAutoProcessContext;

use super::{ast, classify, pattern};

/// Try an oracle-friendly answer before model synthesis.
pub fn answer(
    content: &str,
    query: &str,
    source: String,
    ctx: &CrateAutoProcessContext<'_>,
) -> Result<Option<String>> {
    match classify::classify(ctx.tool_id, query) {
        classify::QueryKind::Pattern => pattern::answer(content, query, source),
        classify::QueryKind::Structural => ast::answer(content, query, source),
        classify::QueryKind::Semantic => Ok(None),
    }
}

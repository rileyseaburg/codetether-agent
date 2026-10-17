//! Count pinned pages and format the existing context-budget report.
use codetether_context_policy::pages::PageKind;
use codetether_tool_core::ToolResult;
pub(super) fn budget(total: usize, cached: usize, pages: &[PageKind]) -> ToolResult {
    let pinned = pages
        .iter()
        .filter(|p| matches!(p, PageKind::Constraint | PageKind::Bootstrap))
        .count();
    let output = format!(
        "L_working={total} messages, cached_summaries={cached}, \
             pinned_pages={pinned}, L_effective≈{eff}",
        eff = total + cached * 50,
    );
    ToolResult::success(output)
}

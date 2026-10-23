//! Render attempts into the stable `websearch` tool output.
use crate::{SearchFailure, attempt::Attempt, result::SearchResult};
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};

/// Prefix of the single-line JSON evidence appended to every tool output.
///
/// # Examples
/// ```rust
/// assert_eq!(codetether_websearch::EVIDENCE_PREFIX, "Search evidence: ");
/// ```
pub const EVIDENCE_PREFIX: &str = "Search evidence: ";

/// Successful attempt output, or the last failure with every attempt recorded.
pub(crate) fn render(query: &str, attempts: &[Attempt]) -> ToolResult {
    let evidence = evidence(query, attempts);
    let line = format!("{EVIDENCE_PREFIX}{evidence}");
    let result = match attempts.iter().find_map(|a| a.outcome.as_ref().ok()) {
        Some(results) if results.is_empty() => {
            ToolResult::success(format!("No results found\n\n{line}"))
                .with_metadata("count", json!(0))
        }
        Some(results) => ToolResult::success(format!("{}\n\n{line}", list(results)))
            .with_metadata("count", json!(results.len())),
        None => match attempts.last().map(|a| &a.outcome) {
            Some(Err(failure)) => failed(failure, &line),
            _ => failed(
                &SearchFailure::Config("no search backend attempted".into()),
                &line,
            ),
        },
    };
    result.with_metadata("search_evidence", evidence)
}

/// Failure raised before any backend was contacted.
pub(crate) fn config_error(query: &str, failure: &SearchFailure) -> ToolResult {
    let evidence = evidence(query, &[]);
    let line = format!("{EVIDENCE_PREFIX}{evidence}");
    failed(failure, &line).with_metadata("search_evidence", evidence)
}

fn failed(failure: &SearchFailure, line: &str) -> ToolResult {
    let text = format!(
        "Web search failed [{}]: {}\n\n{line}",
        failure.code(),
        failure.detail()
    );
    ToolResult::error(text).with_metadata("error_code", json!(failure.code()))
}

fn list(results: &[SearchResult]) -> String {
    let lines = results.iter().enumerate().map(|(i, r)| {
        format!(
            "{}. {}\n   URL: {}\n   {}",
            i + 1,
            r.title,
            r.url,
            r.snippet
        )
    });
    lines.collect::<Vec<_>>().join("\n\n")
}

pub(crate) fn evidence(query: &str, attempts: &[Attempt]) -> Value {
    let used = attempts.iter().find(|a| a.outcome.is_ok());
    let sources: Vec<&str> = used
        .and_then(|a| a.outcome.as_ref().ok())
        .map(|r| r.iter().map(|x| x.url.as_str()).collect())
        .unwrap_or_default();
    json!({
        "tool": "websearch",
        "query": query,
        "backend": used.map(|a| a.backend.id()),
        "http_status": used.and_then(|a| a.status),
        "request_id": used.and_then(|a| a.request_id.as_deref()),
        "sources": sources,
        "attempts": attempts.iter().map(Attempt::evidence).collect::<Vec<_>>(),
    })
}

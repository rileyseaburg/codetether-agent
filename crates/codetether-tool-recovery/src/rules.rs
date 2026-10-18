//! Per-tool recovery rules for truncated tool calls.

/// Tool-specific recovery guidance for a truncated tool call.
///
/// The advice is concrete: it names an explicit small size budget so models
/// that ignore vague "make it smaller" hints (and re-truncate repeatedly) get
/// an actionable ceiling per call.
pub(super) fn retry_rule(tool: &str) -> &'static str {
    match tool {
        "batch" => {
            "Do not call `batch` on the retry; issue at most three individual tool calls with short arguments."
        }
        "write" => {
            "The whole file did not fit in one response. Write a small first chunk with `write` \
             (aim for under ~150 lines / 4000 characters), then append the remaining sections in \
             follow-up `edit` calls, one section per turn."
        }
        "edit" | "multiedit" => {
            "The replacement text did not fit in one response. Make one small, targeted `edit` per \
             turn (aim for under ~120 lines / 3500 characters of new text). Replace a single \
             function/section at a time rather than rewriting large regions at once."
        }
        _ => {
            "Retry with one smaller tool call (aim for under ~4000 characters of arguments); split \
             writes, shell scripts, or long arguments into separate turns."
        }
    }
}

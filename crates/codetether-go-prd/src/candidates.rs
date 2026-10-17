//! PRD response recovery: gather json candidates.

use crate::balanced::extract_balanced_braces;

/// Gather candidate JSON strings from LLM output, ordered by likelihood.
pub(crate) fn gather_json_candidates(text: &str) -> Vec<String> {
    let mut candidates = Vec::new();
    let trimmed = text.trim();

    // 1. Direct: entire response is JSON
    candidates.push(trimmed.to_string());

    // 2. Inside ```json ... ``` fences
    let mut search = text;
    while let Some(start) = search.find("```json") {
        let after = &search[start + 7..];
        if let Some(end) = after.find("```") {
            candidates.push(after[..end].trim().to_string());
        }
        search = &search[start + 7..];
    }

    // 3. Inside ``` ... ``` fences (any language tag)
    search = text;
    while let Some(start) = search.find("```") {
        let after = &search[start + 3..];
        let content_start = after.find('\n').unwrap_or(0);
        let after_tag = &after[content_start..];
        if let Some(end) = after_tag.find("```") {
            candidates.push(after_tag[..end].trim().to_string());
        }
        // Advance past this fence pair
        let skip = start + 3 + content_start + after_tag.find("```").unwrap_or(after_tag.len()) + 3;
        if skip >= search.len() {
            break;
        }
        search = &search[skip..];
    }

    // 4. First `{` to last `}` (greedy brace match)
    if let (Some(start), Some(end)) = (text.find('{'), text.rfind('}'))
        && start < end
    {
        candidates.push(text[start..=end].to_string());
    }

    // 5. Balanced brace extraction starting from first `{`
    if let Some(balanced) = extract_balanced_braces(text) {
        candidates.push(balanced);
    }

    candidates
}

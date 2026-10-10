//! Parse bounded, single-line keyboard text from a completed model response.
use serde::Deserialize;

const FENCE: &str = "```windows-reply\n";

/// Exact text and descriptive target; never mouse coordinates or key commands.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TypingProposal {
    pub text: String,
    pub target: String,
}
fn single_line(text: &str, limit: usize) -> bool {
    !text.trim().is_empty()
        && text.encode_utf16().count() <= limit
        && !text
            .chars()
            .any(|c| c.is_control() || matches!(c, '\u{2028}' | '\u{2029}'))
}
/// Only accept one final labelled fence. Return prose separately from keyboard text.
pub(crate) fn parse(analysis: &str) -> Option<(String, TypingProposal)> {
    if analysis.encode_utf16().count() > 32_000 {
        return None;
    }
    let normalized = analysis.replace("\r\n", "\n");
    let (prose, block) = normalized.split_once(FENCE)?;
    if (!prose.is_empty() && !prose.ends_with('\n')) || block.contains(FENCE) {
        return None;
    }
    let json = block.trim().strip_suffix("\n```")?;
    if json.contains("```") {
        return None;
    }
    let proposal: TypingProposal = serde_json::from_str(json).ok()?;
    if !single_line(&proposal.text, 2000) || !single_line(&proposal.target, 200) {
        return None;
    }
    Some((prose.trim_end().chars().take(12_000).collect(), proposal))
}

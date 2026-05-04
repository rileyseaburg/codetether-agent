//! Speech-to-text dictation and command extraction.

use serde::{Deserialize, Serialize};

/// A parsed voice command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCommand {
    pub intent: VoiceIntent,
    pub raw_transcript: String,
    pub parameters: Vec<String>,
}

/// Recognized voice intents.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VoiceIntent {
    /// Run an autonomous task.
    AutoPilot,
    /// Fix a specific issue.
    FixIssue,
    /// Review a PR.
    ReviewPr,
    /// Ask a question.
    Question,
    /// Status check.
    Status,
    /// Unknown / raw dictation.
    Dictation,
}

/// Parse a voice transcript into a structured command.
pub fn parse_voice_command(transcript: &str) -> VoiceCommand {
    let lower = transcript.to_lowercase();
    let (intent, params) = if lower.starts_with("auto pilot") || lower.starts_with("autopilot") {
        (VoiceIntent::AutoPilot, extract_params(&lower, "auto pilot"))
    } else if lower.starts_with("fix issue") {
        (VoiceIntent::FixIssue, extract_params(&lower, "fix issue"))
    } else if lower.starts_with("review") {
        (VoiceIntent::ReviewPr, extract_params(&lower, "review"))
    } else if lower.starts_with("status") || lower.starts_with("what's the status") {
        (VoiceIntent::Status, vec!["current".to_string()])
    } else if lower.starts_with("ask") || lower.ends_with('?') {
        (VoiceIntent::Question, vec![transcript.to_string()])
    } else {
        (VoiceIntent::Dictation, vec![transcript.to_string()])
    };
    VoiceCommand { intent, raw_transcript: transcript.to_string(), parameters: params }
}

fn extract_params(text: &str, prefix: &str) -> Vec<String> {
    text.strip_prefix(prefix)
        .map(|rest| rest.split_whitespace().map(String::from).collect())
        .unwrap_or_default()
}

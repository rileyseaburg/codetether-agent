//! Parsed voice commands and transcript intent recognition.
//!
//! [parse_voice_command] produces a [VoiceCommand] with a [VoiceIntent].
use serde::{Deserialize, Serialize};
mod parser;
pub use parser::parse_voice_command;

/// A parsed voice command.
///
/// # Examples
/// ```
/// use codetether_voice_core::dictation::{parse_voice_command, VoiceIntent};
/// let command = parse_voice_command("fix issue 42");
/// assert_eq!(command.intent, VoiceIntent::FixIssue);
/// assert_eq!(command.parameters, ["42"]);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCommand {
    /// Recognized action category.
    pub intent: VoiceIntent,
    /// Original, unmodified transcript.
    pub raw_transcript: String,
    /// Action parameters extracted from the transcript.
    pub parameters: Vec<String>,
}

/// Recognized voice intents.
///
/// Variants distinguish autonomous work, issue fixes, PR review, questions,
/// status requests and unrecognized dictation.
///
/// # Examples
/// ```
/// use codetether_voice_core::dictation::{parse_voice_command, VoiceIntent};
/// let command = parse_voice_command("status");
/// match command.intent {
///     VoiceIntent::Status => assert_eq!(command.parameters, ["current"]),
///     other => panic!("unexpected intent: {other:?}"),
/// }
/// ```
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

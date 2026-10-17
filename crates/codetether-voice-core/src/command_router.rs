//! Route parsed voice commands to agent actions.

use super::dictation::VoiceCommand;
use super::dictation::VoiceIntent;

/// Route a voice command to an action string.
///
/// # Arguments
/// * `cmd` - Parsed voice command.
///
/// # Returns
/// Text for the agent action; no action is executed.
///
/// # Examples
/// ```
/// use codetether_voice_core::{command_router::route_command, dictation::parse_voice_command};
/// assert_eq!(route_command(&parse_voice_command("status")), "current status");
/// ```
pub fn route_command(cmd: &VoiceCommand) -> String {
    match cmd.intent {
        VoiceIntent::AutoPilot => {
            let task = cmd.parameters.join(" ");
            format!("/autopilot {}", task)
        }
        VoiceIntent::FixIssue => {
            let issue = cmd.parameters.first().map(|s| s.as_str()).unwrap_or("");
            format!("fix issue #{}", issue)
        }
        VoiceIntent::ReviewPr => {
            let pr = cmd
                .parameters
                .first()
                .map(|s| s.as_str())
                .unwrap_or("latest");
            format!("review PR {}", pr)
        }
        VoiceIntent::Status => "current status".to_string(),
        VoiceIntent::Question => cmd.parameters.first().cloned().unwrap_or_default(),
        VoiceIntent::Dictation => format!("implement: {}", cmd.raw_transcript),
    }
}

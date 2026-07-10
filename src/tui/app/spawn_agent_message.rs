//! System-message construction for spawned agents.

use super::super::prompt::system_prompt;
use super::SpawnArgs;

/// Build the initial system message for a child agent.
pub fn system(args: &SpawnArgs) -> crate::provider::Message {
    crate::provider::Message {
        role: crate::provider::Role::System,
        content: vec![crate::provider::ContentPart::Text {
            text: system_prompt(&args.name, &args.instructions),
        }],
    }
}

//! Source-aware labels for agent transcript message roles.

use crate::provider::Role;
use ratatui::{style::Stylize, text::Span};

/// Origin of messages displayed in an agent transcript.
#[derive(Clone, Copy)]
pub(in crate::tui::ui) enum Source {
    /// A locally managed child-agent conversation.
    ManagedAgent,
    /// A request/response exchange with a LAN A2A peer.
    A2aPeer,
}

impl Source {
    /// Select transcript provenance from an agent snapshot.
    pub(in crate::tui::ui) fn for_remote(remote: bool) -> Self {
        if remote {
            Self::A2aPeer
        } else {
            Self::ManagedAgent
        }
    }
}

/// Render one provider role according to the transcript's actual source.
pub(super) fn role(role: Role, source: Source) -> Span<'static> {
    match (role, source) {
        (Role::User, Source::ManagedAgent) => "USER".bold(),
        (Role::Assistant, Source::ManagedAgent) => "ASSISTANT".cyan().bold(),
        (Role::User, Source::A2aPeer) => "A2A REQUEST".bold(),
        (Role::Assistant, Source::A2aPeer) => "A2A RESPONSE".cyan().bold(),
        (Role::System | Role::Developer, _) => "SYSTEM".dim().bold(),
        (Role::Tool, _) => "TOOL RESULT".magenta().bold(),
    }
}

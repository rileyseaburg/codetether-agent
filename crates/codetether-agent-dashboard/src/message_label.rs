//! Source-aware labels for agent transcript message roles.

use codetether_provider_core::Role;
use ratatui::{style::Stylize, text::Span};

/// Origin of messages displayed in an agent transcript.
/// # Examples
/// ```
/// use codetether_agent_dashboard::messages::Source;
/// let label = match Source::ManagedAgent { Source::ManagedAgent => "local", Source::A2aPeer => "remote" };
/// assert_eq!(label, "local");
/// ```
#[derive(Clone, Copy)]
pub enum Source {
    /// A locally managed child-agent conversation.
    ManagedAgent,
    /// A request/response exchange with a LAN A2A peer.
    A2aPeer,
}

impl Source {
    /// Select transcript provenance from an agent snapshot.
    /// # Examples
    /// ```
    /// use codetether_agent_dashboard::messages::Source;
    /// assert!(matches!(Source::for_remote(true), Source::A2aPeer));
    /// ```
    pub fn for_remote(remote: bool) -> Self {
        if remote {
            Self::A2aPeer
        } else {
            Self::ManagedAgent
        }
    }
}

/// Render one provider role according to the transcript's actual source.
pub(crate) fn role(role: Role, source: Source) -> Span<'static> {
    match (role, source) {
        (Role::User, Source::ManagedAgent) => "USER".bold(),
        (Role::Assistant, Source::ManagedAgent) => "ASSISTANT".cyan().bold(),
        (Role::User, Source::A2aPeer) => "A2A REQUEST".bold(),
        (Role::Assistant, Source::A2aPeer) => "A2A RESPONSE".cyan().bold(),
        (Role::System | Role::Developer, _) => "SYSTEM".dim().bold(),
        (Role::Tool, _) => "TOOL RESULT".magenta().bold(),
    }
}

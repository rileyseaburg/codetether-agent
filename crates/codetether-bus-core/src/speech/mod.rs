//! Typed speech acts and their wire spellings.
mod methods;
#[cfg(test)]
mod tests;

/// The performative (intent) of an [`AgentSpeech`](crate::BusMessage::AgentSpeech).
///
/// Loosely inspired by FIPA-ACL / KQML but trimmed to what multi-agent coding
/// swarms actually need.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeechAct {
    /// Ask another agent to do something.
    Request,
    /// Share a fact / status with no reply expected.
    Inform,
    /// Suggest a plan or option for discussion.
    Propose,
    /// Agree with a prior proposal.
    Accept,
    /// Disagree with a prior proposal.
    Reject,
    /// Stake ownership of a work item to avoid duplicate effort.
    Claim,
    /// Signal a blocker that needs help.
    Blocked,
    /// Report a work item finished.
    Done,
    /// Anything that doesn't fit the above; intent lives in the text.
    Say,
}

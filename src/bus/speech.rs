//! # Agent communication language
//!
//! A small, typed *speech-act* layer on top of the raw [`AgentBus`](super::AgentBus).
//! Where [`BusMessage::AgentMessage`](super::BusMessage::AgentMessage) carries
//! free-form A2A parts, [`BusMessage::AgentSpeech`](super::BusMessage::AgentSpeech)
//! pairs LLM-authored free text with a typed *performative* so the recipient
//! knows the *intent* of the message and can decide how to react.
//!
//! ## Example
//!
//! ```rust
//! use codetether_agent::bus::speech::SpeechAct;
//!
//! let act = SpeechAct::Propose;
//! assert_eq!(act.as_str(), "propose");
//! assert_eq!(SpeechAct::from_str_lenient("DONE"), SpeechAct::Done);
//! assert!(SpeechAct::Blocked.expects_reply());
//! ```

#[path = "speech_act_impl.rs"]
mod speech_act_impl;
#[path = "speech_handle.rs"]
mod speech_handle;
#[cfg(test)]
#[path = "speech_tests.rs"]
mod speech_tests;

/// The performative (intent) of an [`AgentSpeech`](super::BusMessage::AgentSpeech).
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

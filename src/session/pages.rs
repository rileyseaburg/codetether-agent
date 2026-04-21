//! Typed pages for chat-history entries (ClawVM §3).
//!
//! ## Role
//!
//! ClawVM (arXiv:2604.10352) demonstrates that fault elimination in
//! stateful agent harnesses comes from the *enforcement layer* — typed
//! pages with minimum-fidelity invariants and a validated writeback
//! protocol — not from the upgrade heuristic (their LRU ablation
//! proves this). This module is the Phase A scaffolding for that
//! enforcement layer: every entry in [`Session::messages`] is tagged
//! with a [`PageKind`] that declares *what it is* and *how far it can
//! degrade under budget pressure without violating a load-bearing
//! invariant*.
//!
//! ## Scope in Phase A
//!
//! In Phase A we populate the parallel `pages: Vec<PageKind>` sidecar
//! and expose [`classify`] so every new entry picks up a kind at
//! append-time. Enforcement consumers (experimental strategies,
//! compression) come online in Phase B and consult this sidecar
//! before degrading any page.
//!
//! ## Examples
//!
//! ```rust
//! use codetether_agent::provider::{ContentPart, Message, Role};
//! use codetether_agent::session::pages::{PageKind, classify};
//!
//! let system = Message {
//!     role: Role::System,
//!     content: vec![ContentPart::Text { text: "you are…".into() }],
//! };
//! assert_eq!(classify(&system), PageKind::Bootstrap);
//!
//! let tool_result = Message {
//!     role: Role::Tool,
//!     content: vec![ContentPart::ToolResult {
//!         tool_call_id: "call-1".into(),
//!         content: "file contents".into(),
//!     }],
//! };
//! assert_eq!(classify(&tool_result), PageKind::Evidence);
//! ```

use serde::{Deserialize, Serialize};

use crate::provider::{ContentPart, Message, Role};

/// ClawVM page classification for a single chat-history entry.
///
/// Kinds are stable identifiers, not flags — a message has exactly one
/// kind. The degradation path and minimum-fidelity invariant are
/// encoded on [`PageKind`] itself (see
/// [`min_fidelity`](Self::min_fidelity) and
/// [`degradation_path`](Self::degradation_path)).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PageKind {
    /// System prompts and procedural directives. Losing them causes
    /// "forgot its protocol" failures (ClawVM §2).
    Bootstrap,
    /// Hard-pinned invariants the agent must honour — codetether's
    /// `.tasks.jsonl` goal entries map here natively.
    Constraint,
    /// Current objective and step. Live plan state.
    Plan,
    /// Scoped user or project preferences.
    Preference,
    /// Tool outputs. Pointer-safe (Phase B): can degrade to a handle
    /// backed by the MinIO history sink.
    Evidence,
    /// User / assistant conversation spans. The bulk of the transcript.
    Conversation,
}

/// Residency level of a page inside the derived context.
///
/// Multi-resolution representations (ClawVM §3): a page degrades along
/// its [`PageKind::degradation_path`] without ever violating its
/// [`PageKind::min_fidelity`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResidencyLevel {
    /// Verbatim original content.
    Full,
    /// Token-reduced text (e.g. LLMLingua-2 compression).
    Compressed,
    /// Typed fields sufficient to satisfy the page's invariant.
    Structured,
    /// Resolvable handle plus minimal metadata. Backed by the MinIO
    /// history sink for Evidence pages.
    Pointer,
}

impl PageKind {
    /// The minimum-fidelity level this kind is allowed to drop to.
    ///
    /// Compression / eviction strategies that would drop a page below
    /// its minimum fidelity must be **rejected** (ClawVM §3), not
    /// silently applied.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::session::pages::{PageKind, ResidencyLevel};
    ///
    /// assert_eq!(PageKind::Constraint.min_fidelity(), ResidencyLevel::Structured);
    /// assert_eq!(PageKind::Evidence.min_fidelity(), ResidencyLevel::Pointer);
    /// ```
    pub fn min_fidelity(self) -> ResidencyLevel {
        match self {
            PageKind::Bootstrap => ResidencyLevel::Structured,
            PageKind::Constraint => ResidencyLevel::Structured,
            PageKind::Plan => ResidencyLevel::Pointer,
            PageKind::Preference => ResidencyLevel::Pointer,
            PageKind::Evidence => ResidencyLevel::Pointer,
            PageKind::Conversation => ResidencyLevel::Pointer,
        }
    }

    /// The ordered sequence of residency levels this kind is allowed
    /// to visit, from full down to its minimum fidelity.
    ///
    /// Used by Phase B's `DerivePolicy::Incremental` to pick the next
    /// marginal downgrade when the token budget is tight.
    pub fn degradation_path(self) -> &'static [ResidencyLevel] {
        match self {
            PageKind::Bootstrap | PageKind::Constraint => {
                &[ResidencyLevel::Full, ResidencyLevel::Structured]
            }
            PageKind::Plan => &[
                ResidencyLevel::Full,
                ResidencyLevel::Structured,
                ResidencyLevel::Pointer,
            ],
            PageKind::Preference | PageKind::Evidence | PageKind::Conversation => &[
                ResidencyLevel::Full,
                ResidencyLevel::Compressed,
                ResidencyLevel::Structured,
                ResidencyLevel::Pointer,
            ],
        }
    }
}

/// Classify a single chat-history entry.
///
/// Heuristic (Phase A):
///
/// * [`Role::System`] → [`PageKind::Bootstrap`].
/// * [`Role::Tool`] or any content part that is
///   [`ContentPart::ToolResult`] → [`PageKind::Evidence`].
/// * Everything else → [`PageKind::Conversation`].
///
/// [`PageKind::Constraint`], [`PageKind::Plan`], and
/// [`PageKind::Preference`] require explicit producers (the
/// `.tasks.jsonl` loader, swarm objectives, user prefs block) and are
/// not reachable from this syntactic classifier. That is intentional —
/// they are load-bearing invariant tags, not best-effort guesses.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::{ContentPart, Message, Role};
/// use codetether_agent::session::pages::{PageKind, classify};
///
/// let user = Message {
///     role: Role::User,
///     content: vec![ContentPart::Text { text: "hi".into() }],
/// };
/// assert_eq!(classify(&user), PageKind::Conversation);
/// ```
pub fn classify(msg: &Message) -> PageKind {
    if matches!(msg.role, Role::System) {
        return PageKind::Bootstrap;
    }
    if matches!(msg.role, Role::Tool) {
        return PageKind::Evidence;
    }
    for part in &msg.content {
        if matches!(part, ContentPart::ToolResult { .. }) {
            return PageKind::Evidence;
        }
    }
    PageKind::Conversation
}

/// Classify every entry in `messages` and return the parallel array.
///
/// Used at session load to backfill the `pages` sidecar for legacy
/// sessions that predate the Phase A refactor.
pub fn classify_all(messages: &[Message]) -> Vec<PageKind> {
    messages.iter().map(classify).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user(s: &str) -> Message {
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: s.to_string(),
            }],
        }
    }

    fn assistant(s: &str) -> Message {
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: s.to_string(),
            }],
        }
    }

    fn tool_result(id: &str, body: &str) -> Message {
        Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: id.to_string(),
                content: body.to_string(),
            }],
        }
    }

    #[test]
    fn classify_routes_system_to_bootstrap() {
        let msg = Message {
            role: Role::System,
            content: vec![ContentPart::Text {
                text: "you are…".to_string(),
            }],
        };
        assert_eq!(classify(&msg), PageKind::Bootstrap);
    }

    #[test]
    fn classify_routes_tool_results_to_evidence() {
        assert_eq!(
            classify(&tool_result("call-1", "output")),
            PageKind::Evidence
        );
    }

    #[test]
    fn classify_defaults_user_and_assistant_to_conversation() {
        assert_eq!(classify(&user("hi")), PageKind::Conversation);
        assert_eq!(classify(&assistant("reply")), PageKind::Conversation);
    }

    #[test]
    fn min_fidelity_never_drops_below_structured_for_invariant_pages() {
        assert_eq!(
            PageKind::Bootstrap.min_fidelity(),
            ResidencyLevel::Structured
        );
        assert_eq!(
            PageKind::Constraint.min_fidelity(),
            ResidencyLevel::Structured
        );
    }

    #[test]
    fn degradation_paths_start_at_full() {
        for kind in [
            PageKind::Bootstrap,
            PageKind::Constraint,
            PageKind::Plan,
            PageKind::Preference,
            PageKind::Evidence,
            PageKind::Conversation,
        ] {
            assert_eq!(kind.degradation_path()[0], ResidencyLevel::Full);
        }
    }

    #[test]
    fn classify_all_is_parallel() {
        let msgs = vec![user("a"), assistant("b"), tool_result("c", "out")];
        let pages = classify_all(&msgs);
        assert_eq!(pages.len(), 3);
        assert_eq!(pages[0], PageKind::Conversation);
        assert_eq!(pages[1], PageKind::Conversation);
        assert_eq!(pages[2], PageKind::Evidence);
    }
}

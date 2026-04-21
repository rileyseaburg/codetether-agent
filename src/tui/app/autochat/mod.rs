//! Autochat relay integration for the TUI event loop.

pub mod events;
pub mod handler;
pub mod notify;
pub mod request;
pub mod run;
pub mod state;
pub mod summary;
pub mod worker;

pub use handler::handle_autochat_event;

// ─────────────────────────────────────────────────────────────────
// Persona chain for the multi-step relay.
//
// Kept inline so the module tree stays visible to rust-analyzer
// even when it hasn't picked up newly created sibling files yet.
// ─────────────────────────────────────────────────────────────────
pub mod persona {
    /// A single persona step in the relay.
    #[derive(Debug, Clone)]
    pub struct Persona {
        pub name: &'static str,
        pub instructions: &'static str,
    }

    /// Default relay chain used by `/autochat` when no OKR is attached.
    pub fn default_chain() -> &'static [Persona] {
        &[
            Persona {
                name: "architect",
                instructions: "You are the architect. Produce a concrete, numbered plan to execute the task. Keep it under 12 steps and call out risks and concrete files/systems involved.",
            },
            Persona {
                name: "implementer",
                instructions: "You are the implementer. Take the architect's plan and turn it into specific implementation actions: code changes, commands, API calls, or config edits. Be concrete. Cite exact identifiers where possible.",
            },
            Persona {
                name: "reviewer",
                instructions: "You are the reviewer. Evaluate the architect's plan and the implementer's steps for gaps, risks, and missing verification. Produce a final action list with explicit verification checks.",
            },
        ]
    }
}

// ─────────────────────────────────────────────────────────────────
// Provider request builder for a single relay step.
// ─────────────────────────────────────────────────────────────────
pub mod step_request {
    use crate::provider::{CompletionRequest, ContentPart, Message, Role};

    use super::persona::Persona;

    /// Build the request for one persona turn.
    ///
    /// The relay feeds the running baton (previous output) forward as
    /// extra user-message context so each persona can build on it.
    pub fn build_step_request(
        model: String,
        persona: &Persona,
        task: &str,
        baton: &str,
    ) -> CompletionRequest {
        CompletionRequest {
            model,
            messages: vec![system_message(persona), user_message(task, baton)],
            tools: Vec::new(),
            temperature: Some(0.9),
            top_p: Some(0.9),
            max_tokens: Some(1000),
            stop: Vec::new(),
        }
    }

    fn system_message(persona: &Persona) -> Message {
        Message {
            role: Role::System,
            content: vec![ContentPart::Text {
                text: persona.instructions.to_string(),
            }],
        }
    }

    fn user_message(task: &str, baton: &str) -> Message {
        let mut text = format!("Task:\n{task}\n");
        if !baton.trim().is_empty() {
            text.push_str("\nPrevious relay output:\n");
            text.push_str(baton);
        }
        Message {
            role: Role::User,
            content: vec![ContentPart::Text { text }],
        }
    }
}

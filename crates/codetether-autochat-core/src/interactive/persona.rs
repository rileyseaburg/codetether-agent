//! Persona chain for the multi-step relay.

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

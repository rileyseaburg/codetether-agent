//! Fallback profile constructors used when no named profile matches.

use super::profiles::AgentProfile;

pub(crate) fn profile_navigator() -> AgentProfile {
    AgentProfile {
        codename: "Navigator",
        profile: "Generalist coordinator",
        personality: "adaptable and context-aware",
        collaboration_style: "balances speed with clarity",
        signature_move: "keeps team momentum aligned",
    }
}

pub(crate) fn profile_vector() -> AgentProfile {
    AgentProfile {
        codename: "Vector",
        profile: "Execution operator",
        personality: "focused and deadline-driven",
        collaboration_style: "prefers direct action and feedback loops",
        signature_move: "drives ambiguous tasks toward decisions",
    }
}

pub(crate) fn profile_signal() -> AgentProfile {
    AgentProfile {
        codename: "Signal",
        profile: "Communication specialist",
        personality: "clear, friendly, and structured",
        collaboration_style: "frames updates for quick handoffs",
        signature_move: "turns noisy context into clean status",
    }
}

pub(crate) fn profile_kernel() -> AgentProfile {
    AgentProfile {
        codename: "Kernel",
        profile: "Core-systems thinker",
        personality: "analytical and stable",
        collaboration_style: "organizes work around constraints and invariants",
        signature_move: "locks down the critical path early",
    }
}

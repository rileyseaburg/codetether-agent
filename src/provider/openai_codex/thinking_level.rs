//! OpenAI Codex reasoning-effort wire values.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ThinkingLevel {
    NoReasoning,
    Low,
    Medium,
    High,
    XHigh,
    Max,
    Ultra,
}

impl ThinkingLevel {
    pub(super) fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "none" => Some(Self::NoReasoning),
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            "xhigh" => Some(Self::XHigh),
            "max" => Some(Self::Max),
            "ultra" => Some(Self::Ultra),
            _ => None,
        }
    }

    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::NoReasoning => "none",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::XHigh => "xhigh",
            Self::Max => "max",
            Self::Ultra => "ultra",
        }
    }
}

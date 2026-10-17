//! OpenAI Codex reasoning-effort wire values.

/// Reasoning effort accepted by Codex model aliases.
///
/// # Examples
/// ```
/// use codetether_provider_core::codex::ThinkingLevel;
/// let level = ThinkingLevel::parse("ultra").unwrap();
/// assert_eq!(level.as_str(), "ultra");
/// assert_eq!(level.as_wire_str(), "max");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThinkingLevel {
    NoReasoning,
    Low,
    Medium,
    High,
    XHigh,
    Max,
    Ultra,
}

impl ThinkingLevel {
    /// Parse a case-insensitive selector effort.
    pub fn parse(raw: &str) -> Option<Self> {
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

    /// Return the selector spelling for this effort.
    pub fn as_str(self) -> &'static str {
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

    /// Return the upstream spelling, translating Ultra to Max.
    pub fn as_wire_str(self) -> &'static str {
        match self {
            Self::Ultra => Self::Max.as_str(),
            level => level.as_str(),
        }
    }
}

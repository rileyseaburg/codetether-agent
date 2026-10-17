//! Rotating thought phases driving the cognition loop.

use super::ThoughtEventType;

/// The phase a persona occupies on a given tick.
///
/// Variants are Observe, Reflect, Test and Compress.
///
/// # Examples
/// ```
/// use codetether_cognition_core::ThoughtPhase;
/// match ThoughtPhase::from_thought_count(1) {
///     ThoughtPhase::Observe => {},
///     other => panic!("unexpected phase: {other:?}"),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThoughtPhase {
    Observe,
    Reflect,
    Test,
    Compress,
}

impl ThoughtPhase {
    /// Rotate phases so each persona cycles observe → reflect → test → compress.
    ///
    /// # Examples
    /// ```
    /// use codetether_cognition_core::ThoughtPhase;
    /// assert_eq!(ThoughtPhase::from_thought_count(4), ThoughtPhase::Compress);
    /// ```
    pub fn from_thought_count(thought_count: u64) -> Self {
        match thought_count % 4 {
            1 => Self::Observe,
            2 => Self::Reflect,
            3 => Self::Test,
            _ => Self::Compress,
        }
    }

    /// Stable label for logs and payloads.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Observe => "observe",
            Self::Reflect => "reflect",
            Self::Test => "test",
            Self::Compress => "compress",
        }
    }

    /// Event type emitted when this phase produces a thought.
    pub fn event_type(&self) -> ThoughtEventType {
        match self {
            Self::Observe => ThoughtEventType::ThoughtGenerated,
            Self::Reflect => ThoughtEventType::HypothesisRaised,
            Self::Test => ThoughtEventType::CheckRequested,
            Self::Compress => ThoughtEventType::SnapshotCompressed,
        }
    }
}

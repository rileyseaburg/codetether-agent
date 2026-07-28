//! Healing candidate types.

/// Stable path to an element in a DOM tree, represented by child indexes.
pub type ElementPath = Vec<usize>;

/// Strategy that produced a healing candidate.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum HealStrategy {
    AttrRecovery,
    StructuralPath,
    TextProximity,
    RoleBased,
    SiblingProximity,
    PositionHint,
    Generated,
}

/// Possible replacement selector produced by selector healing.
#[derive(Clone, Debug)]
pub struct SelectorCandidate {
    pub selector: String,
    pub confidence: f32,
    pub strategy: HealStrategy,
    pub matches: Vec<ElementPath>,
}

impl SelectorCandidate {
    pub fn new(
        selector: impl Into<String>,
        confidence: f32,
        strategy: HealStrategy,
        matches: Vec<ElementPath>,
    ) -> Self {
        Self {
            selector: selector.into(),
            confidence: confidence.clamp(0.0, 1.0),
            strategy,
            matches,
        }
    }

    pub fn is_unique(&self) -> bool {
        self.matches.len() == 1
    }
}

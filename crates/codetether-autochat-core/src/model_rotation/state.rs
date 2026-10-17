//! Serializable state for rotating among model references.
use super::*;

/// Ordered provider/model references with a saturating round-robin cursor.
///
/// # Examples
/// ```
/// use codetether_autochat_core::model_rotation::RelayModelRotation;
/// let mut rotation = RelayModelRotation::fallback("provider/model");
/// assert_eq!(rotation.next_model_ref("fallback"), "provider/model");
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RelayModelRotation {
    #[serde(default)]
    /// Ordered provider/model choices.
    pub model_refs: Vec<String>,
    #[serde(default)]
    /// Index advanced after each selection, saturating on overflow.
    pub cursor: usize,
}

impl RelayModelRotation {
    /// Construct a one-model rotation starting at index zero.
    pub fn fallback(model_ref: &str) -> Self {
        Self {
            model_refs: vec![model_ref.to_string()],
            cursor: 0,
        }
    }

    /// Return the next choice, or the supplied fallback for an empty rotation.
    pub fn next_model_ref(&mut self, fallback: &str) -> String {
        if self.model_refs.is_empty() {
            return fallback.to_string();
        }
        let idx = self.cursor % self.model_refs.len();
        let next = self.model_refs[idx].clone();
        self.cursor = self.cursor.saturating_add(1);
        next
    }
}

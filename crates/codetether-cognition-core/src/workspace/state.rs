//! The coherent shared "now" for the entire swarm.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Cross-persona summary of current beliefs, gaps, and objectives.
///
/// # Examples
///
/// ```rust
/// use codetether_cognition_core::GlobalWorkspace;
/// let workspace = GlobalWorkspace::default();
/// assert!(workspace.top_beliefs.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalWorkspace {
    /// Selected belief identifiers.
    pub top_beliefs: Vec<String>,
    /// Selected unresolved uncertainty text.
    pub top_uncertainties: Vec<String>,
    /// Selected attention identifiers.
    pub top_attention: Vec<String>,
    /// Current shared objectives.
    pub active_objectives: Vec<String>,
    /// Most recent state update time.
    pub updated_at: DateTime<Utc>,
}

impl Default for GlobalWorkspace {
    fn default() -> Self {
        Self {
            top_beliefs: Vec::new(),
            top_uncertainties: Vec::new(),
            top_attention: Vec::new(),
            active_objectives: Vec::new(),
            updated_at: Utc::now(),
        }
    }
}

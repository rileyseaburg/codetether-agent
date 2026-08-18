//! The coherent shared "now" for the entire swarm.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Cross-persona summary of current beliefs, gaps, and objectives.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cognition::GlobalWorkspace;
/// let workspace = GlobalWorkspace::default();
/// assert!(workspace.top_beliefs.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalWorkspace {
    pub top_beliefs: Vec<String>,
    pub top_uncertainties: Vec<String>,
    pub top_attention: Vec<String>,
    pub active_objectives: Vec<String>,
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

//! Peer cost card — advertized capabilities for mesh scheduling.

use serde::{Deserialize, Serialize};

/// Capabilities advertized by a mesh peer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostCard {
    pub peer_id: String,
    pub hostname: String,
    pub free_cores: usize,
    pub gpu_available: bool,
    pub gpu_model: Option<String>,
    pub available_models: Vec<String>,
    pub cost_per_1k_tokens: f32,
    pub max_concurrent_tasks: usize,
    pub current_load: f32,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CostCard {
    /// Compute a scheduling score (lower = more attractive).
    pub fn scheduling_score(&self) -> f32 {
        let load_penalty = self.current_load * 0.4;
        let core_bonus = (1.0 / (self.free_cores as f32 + 1.0)) * 0.2;
        let cost = self.cost_per_1k_tokens * 0.3;
        let gpu_bonus = if self.gpu_available { -0.1 } else { 0.0 };
        load_penalty + core_bonus + cost + gpu_bonus
    }
}

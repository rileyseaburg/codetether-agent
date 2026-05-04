//! Per-agent reputation tracking for auction bidding.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Reputation record for a single agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationRecord {
    pub agent_id: String,
    pub tasks_completed: u32,
    pub tasks_failed: u32,
    pub avg_quality_score: f32,
    pub total_earnings: f32,
}

impl ReputationRecord {
    pub fn reputation_score(&self) -> f32 {
        if self.tasks_completed + self.tasks_failed == 0 {
            return 0.5;
        }
        let success_rate = self.tasks_completed as f32
            / (self.tasks_completed + self.tasks_failed) as f32;
        (success_rate * 0.6 + self.avg_quality_score * 0.4).clamp(0.0, 1.0)
    }
}

/// Simple in-memory reputation ledger.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReputationLedger {
    pub records: HashMap<String, ReputationRecord>,
}

impl ReputationLedger {
    pub fn get_or_create(&mut self, agent_id: &str) -> &mut ReputationRecord {
        self.records
            .entry(agent_id.to_string())
            .or_insert_with(|| ReputationRecord {
                agent_id: agent_id.to_string(),
                tasks_completed: 0,
                tasks_failed: 0,
                avg_quality_score: 0.5,
                total_earnings: 0.0,
            })
    }

    pub fn record_success(&mut self, agent_id: &str, quality: f32) {
        let rec = self.get_or_create(agent_id);
        rec.tasks_completed += 1;
        rec.avg_quality_score = (rec.avg_quality_score + quality) / 2.0;
    }
}

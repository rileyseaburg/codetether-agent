//! Agent bid on an OKR work item.

use serde::{Deserialize, Serialize};

/// A bid submitted by an agent for a KR auction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentBid {
    pub agent_id: String,
    pub plan_summary: String,
    pub estimated_cost: f32,
    pub expected_confidence: f32,
    pub model_class: String,
    pub reputation_score: f32,
    pub proposed_duration_secs: u64,
}

impl AgentBid {
    /// Compute expected value: confidence * reputation / cost.
    pub fn expected_value(&self) -> f32 {
        if self.estimated_cost <= 0.0 {
            return 0.0;
        }
        (self.expected_confidence * self.reputation_score) / self.estimated_cost
    }
}

/// Create a bid from an agent's self-assessment.
pub fn create_bid(agent_id: &str, plan: &str, model: &str, reputation: f32) -> AgentBid {
    AgentBid {
        agent_id: agent_id.to_string(),
        plan_summary: plan.to_string(),
        estimated_cost: 1.0,
        expected_confidence: 0.7,
        model_class: model.to_string(),
        reputation_score: reputation,
        proposed_duration_secs: 300,
    }
}

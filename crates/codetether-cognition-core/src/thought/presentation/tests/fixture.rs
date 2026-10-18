//! Fixed thought-work input for normalization regressions.
use crate::{ThoughtPhase, ThoughtWorkItem};

/// A work item with a fixed persona, varying only by phase.
pub(super) fn sample_work_item(phase: ThoughtPhase) -> ThoughtWorkItem {
    ThoughtWorkItem {
        persona_id: "p-1".to_string(),
        persona_name: "Spotlessbinco Business Thinker".to_string(),
        role: "principal reliability engineer".to_string(),
        charter:
            "Continuously think about /home/riley/spotlessbinco as a production business system."
                .to_string(),
        swarm_id: Some("spotlessbinco".to_string()),
        thought_count: 4,
        phase,
    }
}

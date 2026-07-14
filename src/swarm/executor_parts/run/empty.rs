//! Result returned when decomposition yields no work.

use crate::swarm::{SwarmResult, SwarmStats};

pub(super) fn result() -> SwarmResult {
    SwarmResult {
        success: false,
        result: String::new(),
        subtask_results: Vec::new(),
        stats: SwarmStats::default(),
        artifacts: Vec::new(),
        error: Some("No subtasks generated".to_string()),
    }
}

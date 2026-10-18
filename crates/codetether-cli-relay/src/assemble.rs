//! Initial counters and baton for a newly registered relay.
use crate::{Host, initial::Initial, state::Runtime};
use std::collections::HashMap;
impl<H: Host> Runtime<H> {
    pub(super) fn assemble(initial: Initial<H>, task: &str) -> Self {
        let Initial {
            bus,
            relay,
            registry,
            team,
            context_receiver,
            shared_context,
            kr_targets,
            planner_used,
        } = initial;
        Self {
            bus,
            relay,
            registry,
            team,
            context_receiver,
            shared_context,
            kr_targets,
            planner_used,
            kr_progress: HashMap::new(),
            baton: format!(
                "Task:\n{task}\n\nStart by proposing an execution strategy and one immediate next step."
            ),
            previous_normalized: None,
            convergence_hits: 0,
            turns: 0,
            dynamic_spawn_count: 0,
            rlm_handoff_count: 0,
            rlm_context_count: 0,
            status: codetether_autochat_core::AUTOCHAT_STATUS_MAX_ROUNDS_REACHED.to_string(),
            failure_note: None,
        }
    }
}

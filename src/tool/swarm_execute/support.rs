//! Support helpers for `swarm_execute`: sub-agent tool filtering and
//! shared tool filtering for autonomous sub-agents.
//!
use crate::tool::ToolRegistry;

/// Tool definitions exposed to swarm sub-agents.
///
/// Interactive and recursive tools are removed because sub-agents run
/// autonomously and must not spawn further swarms.
pub(super) fn subagent_tools() -> Vec<crate::provider::ToolDefinition> {
    ToolRegistry::new()
        .definitions()
        .into_iter()
        .filter(|t| {
            !matches!(
                t.name.as_str(),
                "question"
                    | "confirm_edit"
                    | "confirm_multiedit"
                    | "plan_enter"
                    | "plan_exit"
                    | "swarm_execute"
                    | "agent"
            )
        })
        .collect()
}

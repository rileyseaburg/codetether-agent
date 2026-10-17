//! Tests for the agent registry.

use super::{AgentInfo, AgentMode, AgentRegistry};

#[test]
fn with_builtins_registers_expected_agents() {
    let registry = AgentRegistry::with_builtins();
    let mut names = registry
        .list()
        .into_iter()
        .map(|agent| agent.name.as_str())
        .collect::<Vec<_>>();
    names.sort_unstable();

    assert_eq!(names, vec!["build", "explore", "plan"]);
}

#[test]
fn list_primary_excludes_hidden_and_subagents() {
    let mut registry = AgentRegistry::new();
    registry.register(agent_info("build", AgentMode::Primary, false));
    registry.register(agent_info("hidden", AgentMode::Primary, true));
    registry.register(agent_info("explore", AgentMode::Subagent, false));

    let mut names = registry
        .list_primary()
        .into_iter()
        .map(|agent| agent.name.as_str())
        .collect::<Vec<_>>();
    names.sort_unstable();

    assert_eq!(names, vec!["build"]);
}

fn agent_info(name: &str, mode: AgentMode, hidden: bool) -> AgentInfo {
    AgentInfo {
        name: name.to_string(),
        description: None,
        mode,
        native: true,
        hidden,
        model: None,
        temperature: None,
        top_p: None,
        max_steps: None,
    }
}

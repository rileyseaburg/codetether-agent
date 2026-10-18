//! Stable ordered union of managed, tool and swarm agent names.
/// Preserve managed order, sort tool agents, then append unseen swarm names.
///
/// # Arguments
/// * `names` — Managed agents in depth-first traversal order.
/// * `tools` — Observed tool-agent names to sort and merge.
/// * `swarm` — Optional agent names in swarm task order.
///
/// # Returns
/// One stable ring with duplicates suppressed for appended sources.
///
/// # Examples
/// ```
/// let names = codetether_tui_navigation::focus::assemble(
///     vec!["managed".into()], vec!["tool".into()],
///     vec![Some("tool".into()), Some("swarm".into())],
/// );
/// assert_eq!(names, vec!["managed", "tool", "swarm"]);
/// ```
pub fn assemble(
    mut names: Vec<String>,
    mut tools: Vec<String>,
    swarm: Vec<Option<String>>,
) -> Vec<String> {
    tools.sort_by(|left, right| left.cmp(right));
    for tool in tools {
        if !names.contains(&tool) {
            names.push(tool);
        }
    }
    for agent_name in swarm {
        if let Some(name) = agent_name.as_ref()
            && !names.contains(name)
        {
            names.push(name.clone());
        }
    }
    names
}

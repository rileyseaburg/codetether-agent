//! Resolve unique swarm worker prefixes for agent focus commands.
use crate::State;
/// Focus a uniquely matching swarm worker.
/// # Arguments
/// `app` supplies ordered worker names and detail-view selection; `query` is a name/prefix.
/// # Returns
/// True when a worker was selected or an ambiguous-prefix status was displayed.
/// # Examples
/// ```no_run
/// fn command<H: codetether_agent_command::State>(host: &mut H) {
///     codetether_agent_command::focus_swarm(host, "agent-");
/// }
/// ```
pub fn focus<H: State>(app: &mut H, query: &str) -> bool {
    let names = app.swarm_names();
    let matches = names
        .iter()
        .enumerate()
        .filter(|(_, name)| {
            name.as_deref()
                .is_some_and(|name| name == query || name.starts_with(query))
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let [index] = matches.as_slice() else {
        if matches.len() > 1 {
            app.status(format!("Ambiguous swarm agent prefix: {query}"));
            return true;
        }
        return false;
    };
    app.select_swarm(*index);
    app.status(format!("Focused swarm agent: @{query}"));
    true
}

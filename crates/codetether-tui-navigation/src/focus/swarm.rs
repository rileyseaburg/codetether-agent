//! Agent focus swarm policy.
use super::Host;

pub(super) fn sync(app: &mut impl Host, name: Option<&str>) {
    let Some(index) = name.and_then(|name| {
        app.swarm_names()
            .iter()
            .position(|agent| agent.as_deref() == Some(name))
    }) else {
        return;
    };
    app.select_swarm_agent(index);
}

//! Registration of an independent detached agent.
use super::Host;
use codetether_agent_tree::SpawnedAgent;
/// Store a detached session as a background sub-agent.
pub(super) fn register_detached<H: Host>(app: &mut H, name: &str, child: H::Session) {
    app.register(
        name.to_string(),
        SpawnedAgent {
            name: name.to_string(),
            instructions: String::new(),
            parent: None,
            depth: 0,
            session: child,
            model_id: app.current_model_id(),
            is_processing: false,
        },
    );
}

//! Registration helper for detached chat threads.

use crate::session::Session;
use crate::tui::app::spawn_agent::model::current_model_id;
use crate::tui::app::state::{App, SpawnedAgent};

/// Store a detached session as a background sub-agent.
pub fn register_detached(app: &mut App, name: &str, child: Session) {
    app.state.spawned_agents.insert(
        name.to_string(),
        SpawnedAgent {
            name: name.to_string(),
            instructions: String::new(),
            parent: None,
            depth: 0,
            session: child,
            model_id: current_model_id(app),
            is_processing: false,
        },
    );
}

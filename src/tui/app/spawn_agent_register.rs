//! Registration helper for parent-deployed sub-agents.

use super::SpawnArgs;
use crate::session::Session;
use crate::tui::app::state::{App, SpawnedAgent};

/// Store a child agent in the parent TUI registry.
pub fn insert(app: &mut App, args: SpawnArgs, depth: u8, session: Session) {
    app.state.spawned_agents.insert(
        args.name.clone(),
        SpawnedAgent {
            name: args.name,
            instructions: args.instructions,
            parent: args.parent,
            depth,
            session,
            model_id: super::super::model::current_model_id(app),
            is_processing: false,
        },
    );
}

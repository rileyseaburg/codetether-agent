//! `/spawn` adapter for the canonical managed-agent runtime.

use std::path::Path;

use crate::session::Session;
use crate::tui::app::state::App;

#[path = "spawn_agent_parse.rs"]
mod args;
#[path = "spawn_agent_model.rs"]
pub mod model;
#[path = "spawn_agent_prompt.rs"]
mod prompt;

/// Spawn and immediately dispatch a managed child agent.
pub async fn handle_spawn_command(app: &mut App, session: &Session, cwd: &Path, rest: &str) {
    let Some(args) = args::parse(rest) else {
        app.state.status = "Usage: /spawn <name> [instructions]".to_string();
        return;
    };
    if args.parent.is_some() {
        app.state.status =
            "Nested --parent spawning is not supported by the managed runtime".into();
        return;
    }
    let instructions = prompt::system_prompt(&args.name, &args.instructions);
    let result =
        crate::tui::app::managed_agent::spawn(session, cwd, &args.name, &instructions).await;
    if result.success {
        app.state.active_spawned_agent = Some(args.name.clone());
        app.state
            .set_view_mode(crate::tui::models::ViewMode::Subagents);
    }
    crate::tui::app::managed_agent::ui::present(
        app,
        &result,
        format!("Managed sub-agent @{} started", args.name),
        "Sub-agent spawn failed",
    );
}

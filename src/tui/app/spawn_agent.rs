//! `/spawn` handler with parent lineage and depth-capped recursion.
//!
//! Supports `/spawn <name> [--parent <p>] [instructions]`, building the
//! spawned-agent tree. Depth is enforced via
//! [`crate::tui::app::state::agent_spawn_guard::validate_spawn`] so an agent
//! that spawns agents cannot exceed
//! [`crate::tui::app::state::agent_tree::MAX_AGENT_DEPTH`].

use crate::tui::app::state::App;
use crate::tui::app::state::agent_spawn_guard::validate_spawn;

#[path = "spawn_agent_create.rs"]
mod create;
#[path = "spawn_agent_model.rs"]
pub mod model;
#[path = "spawn_agent_prompt.rs"]
mod prompt;
use create::create_agent;

/// Parsed `/spawn` arguments: name, optional parent, instructions.
pub(super) struct SpawnArgs {
    pub name: String,
    pub parent: Option<String>,
    pub instructions: String,
}

/// Parse `<name> [--parent <p>] [instructions]` from the raw argument string.
fn parse_args(rest: &str) -> Option<SpawnArgs> {
    let mut tokens = rest.trim().split_whitespace().peekable();
    let name = tokens.next()?.to_string();
    let mut parent = None;
    if tokens.peek() == Some(&"--parent") {
        tokens.next();
        parent = tokens.next().map(str::to_string);
    }
    let instructions = tokens.collect::<Vec<_>>().join(" ");
    Some(SpawnArgs {
        name,
        parent,
        instructions,
    })
}

/// Handle `/spawn <name> [--parent <p>] [instructions]`.
pub async fn handle_spawn_command(app: &mut App, rest: &str) {
    let Some(args) = parse_args(rest) else {
        app.state.status = "Usage: /spawn <name> [--parent <p>] [instructions]".to_string();
        return;
    };
    if app.state.spawned_agents.contains_key(&args.name) {
        app.state.status = format!("Agent '{}' already exists. /kill it first.", args.name);
        return;
    }
    let depth = match validate_spawn(&app.state.spawned_agents, args.parent.as_deref()) {
        Ok(d) => d,
        Err(e) => {
            create::note(app, e.clone());
            app.state.status = e;
            return;
        }
    };
    create_agent(app, args, depth).await;
}

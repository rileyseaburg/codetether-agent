//! User-facing notices for sub-agent deployment.

use super::create::note;
use crate::tui::app::state::App;

/// Record that the parent deployed a child agent.
pub fn deployed(app: &mut App, name: &str, lineage: &str) {
    app.state.status = format!("Deployed subagent: {name}{lineage}");
    note(
        app,
        format!(
            "Parent deployed subagent '{name}'{lineage}. Open /agents to watch all children and report-back state."
        ),
    );
}

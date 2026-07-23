//! Detail-row assembly for `/spawn`-managed children.

use ratatui::text::Line;

use crate::tui::app::state::SpawnedAgent;

pub(super) fn lines(agent: &SpawnedAgent) -> Vec<Line<'static>> {
    let status = if agent.is_processing {
        "working"
    } else {
        "idle"
    };
    let model = agent.model_id.as_deref().unwrap_or("default model");
    let parent = agent.parent.as_deref().unwrap_or("main");
    let mut rows = super::subagent_detail_metadata::lines(
        &agent.name,
        parent,
        status,
        model,
        &agent.instructions,
    );
    super::subagent_message_lines::append(
        &mut rows,
        &agent.session.messages,
        super::subagent_message_lines::Source::ManagedAgent,
    );
    rows
}

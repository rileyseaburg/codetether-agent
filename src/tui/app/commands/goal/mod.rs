//! Human-facing `/goal` command routing.

mod auto;
mod budget;
mod show;
mod status;
mod write;

use crate::session::tasks::GoalEditAction;
use crate::tui::app::state::App;

pub(crate) async fn handle(app: &mut App, session_id: &str, raw: &str) -> bool {
    let raw = raw.trim();
    let (verb, tail) = raw
        .split_once(char::is_whitespace)
        .map_or((raw, ""), |(verb, tail)| (verb, tail.trim()));
    let result = match verb {
        "" | "show" | "status" => show::run(session_id).await,
        "auto" => auto::run(tail),
        "set" => write::set(session_id, tail).await,
        "edit" => write::edit(session_id, tail).await,
        "override" => {
            write::change(
                session_id,
                GoalEditAction::Override,
                (!tail.is_empty()).then(|| tail.to_string()),
                None,
            )
            .await
        }
        "budget" => budget::run(session_id, tail).await,
        "reaffirm" => write::reaffirm(session_id, tail).await,
        "clear" => write::clear(session_id, tail).await,
        "done" => status::set(session_id, "complete").await,
        "pause" => status::set(session_id, "paused").await,
        "resume" => status::set(session_id, "active").await,
        other => Err(anyhow::anyhow!(
            "unknown /goal subcommand `{other}`; use set, auto, edit, override, budget, pause, resume, done, clear, or show (/tasks lists work items)"
        )),
    };
    match result {
        Ok(message) => {
            super::push_system_message(app, message.clone());
            app.state.status = message.lines().next().unwrap_or("Goal updated").to_string();
            true
        }
        Err(error) => {
            app.state.status = format!("/goal: {error}");
            false
        }
    }
}

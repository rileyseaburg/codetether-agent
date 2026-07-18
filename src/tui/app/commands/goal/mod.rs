//! Human-facing `/goal` command routing.

mod show;
mod status;
mod write;

use crate::session::Session;
use crate::tui::app::state::App;

pub(super) async fn handle(app: &mut App, session: &Session, raw: &str) {
    let raw = raw.trim();
    let (verb, tail) = raw
        .split_once(char::is_whitespace)
        .map_or((raw, ""), |(verb, tail)| (verb, tail.trim()));
    let result = match verb {
        "" | "show" | "status" => show::run(&session.id).await,
        "set" => write::set(session, tail).await,
        "edit" => write::edit(&session.id, tail).await,
        "reaffirm" => write::reaffirm(&session.id, tail).await,
        "clear" => write::clear(&session.id, tail).await,
        "done" => status::set(&session.id, "complete").await,
        "pause" => status::set(&session.id, "paused").await,
        "resume" => status::set(&session.id, "active").await,
        other => Err(anyhow::anyhow!(
            "unknown /goal subcommand `{other}`; use set, edit, pause, resume, done, clear, or show"
        )),
    };
    match result {
        Ok(message) => {
            super::push_system_message(app, message.clone());
            app.state.status = message.lines().next().unwrap_or("Goal updated").to_string();
        }
        Err(error) => app.state.status = format!("/goal: {error}"),
    }
}

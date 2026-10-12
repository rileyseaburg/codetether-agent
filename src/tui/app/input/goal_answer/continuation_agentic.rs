//! Turn `/ask` on a held goal into a normal, tool-capable continuation.

use crate::session::tasks::runtime::answer_review;
use crate::tui::app::state::App;

pub async fn release_ask(app: &mut App, session_id: &str) -> anyhow::Result<bool> {
    if !answer_review::held(session_id) {
        return Ok(false);
    }
    let input = app.state.input.trim();
    let Some(suffix) = input.strip_prefix("/ask") else {
        return Ok(false);
    };
    if suffix.is_empty() || !suffix.starts_with(char::is_whitespace) {
        return Ok(false);
    }
    let request = suffix.trim();
    if request.is_empty() {
        return Ok(false);
    }
    let request = request.to_string();
    if !super::release::release(session_id).await? {
        return Ok(false);
    }
    app.state.input_cursor = request.chars().count();
    app.state.input = request;
    app.state.answer_review_yes = false;
    Ok(true)
}

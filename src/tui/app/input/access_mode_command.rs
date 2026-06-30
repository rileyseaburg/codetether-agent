use std::path::Path;
use std::sync::Arc;

use crate::config::Config;
use crate::provider::ProviderRegistry;
use crate::session::Session;
use crate::tui::app::state::App;

#[path = "access_mode_parse.rs"]
mod parse;
#[path = "access_mode_policy.rs"]
mod policy;
#[path = "access_mode_result.rs"]
mod result;
#[path = "access_mode_session.rs"]
mod session_config;

pub(super) async fn run(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: Option<&Arc<ProviderRegistry>>,
    prompt: &str,
) -> bool {
    let Some((command, rest)) = parse::prompt(prompt) else {
        return false;
    };
    let rest = rest.trim();
    if rest.is_empty() && command == "/permissions" {
        return false;
    }
    let Some(mode) = parse::mode(rest) else {
        app.state.status = "Usage: /access-mode <ask|approve|full>".to_string();
        return true;
    };
    Config::apply_process_access_mode_override(Some(mode));
    session_config::apply(cwd, session, registry).await;
    result::push(app, cwd, mode).await;
    true
}

//! Native session projection for the shared status report.
#[path = "codex_parity_status_host.rs"]
mod host;
use crate::{session::Session, tui::app::state::App};
use std::path::Path;
pub(super) async fn show(app: &mut App, cwd: &Path, session: &Session, label: &str) {
    codetether_chat_status::command::show(&mut host::Native { app, session }, cwd, label).await;
}

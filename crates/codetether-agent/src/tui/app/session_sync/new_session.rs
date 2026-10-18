//! Native bindings for shared new-session creation.
mod session;
mod view;
use crate::{session::Session, tui::app::state::App};
use std::path::Path;
/// Create and activate a fresh session through the shared lifecycle.
pub async fn create(app: &mut App, cwd: &Path, session: &mut Session) {
    codetether_new_session::create(&mut view::NewView(app), cwd, session).await;
}

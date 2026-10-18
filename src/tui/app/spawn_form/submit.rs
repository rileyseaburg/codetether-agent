//! Native session binding for shared form submission.
use super::SpawnFormState;
use crate::{
    session::Session,
    tui::app::{spawn_agent::handle_spawn_command, state::App},
};
use std::path::Path;
struct Native<'a> {
    app: &'a mut App,
    session: &'a Session,
    cwd: &'a Path,
}
impl codetether_spawn_form::submission::Host for Native<'_> {
    fn status(&mut self, text: String) {
        self.app.state.status = text;
    }
    fn restore(&mut self, form: SpawnFormState) {
        self.app.state.spawn_form = Some(form);
    }
    async fn spawn(&mut self, args: &str) {
        handle_spawn_command(self.app, self.session, self.cwd, args).await;
    }
    fn is_open(&self) -> bool {
        self.app.state.spawn_form.is_some()
    }
    fn close(&mut self) {
        self.app.state.spawn_form = None;
    }
}
/// Submit the form through the session-bound managed spawn command.
pub async fn submit_spawn_form(app: &mut App, session: &Session, cwd: &Path, form: SpawnFormState) {
    codetether_spawn_form::submission::submit(&mut Native { app, session, cwd }, form).await;
}

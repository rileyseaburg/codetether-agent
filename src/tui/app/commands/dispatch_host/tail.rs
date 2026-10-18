//! Native trailing agent and autochat command operations.
use super::*;
pub(super) async fn run(host: &mut Dispatch<'_>, kind: TailCommand, rest: &str) {
    let Dispatch {
        app, cwd, session, ..
    } = host;
    match kind {
        TailCommand::SpawnForm => crate::tui::app::spawn_form::open_spawn_form(app),
        TailCommand::SpawnTask => handle_spawn_command(app, session, cwd, rest).await,
        TailCommand::Kill => {
            crate::tui::app::managed_agent::command::admin::kill(app, session, rest).await
        }
        TailCommand::Agents => crate::tui::app::managed_agent::command::admin::list(app),
        TailCommand::Agent => {
            crate::tui::app::managed_agent::command::handle(app, session, rest).await
        }
        TailCommand::Autochat => handle_autochat_command(app, rest),
    }
}

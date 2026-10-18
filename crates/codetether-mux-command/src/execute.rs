//! Execute parsed `/mux` actions against the shared control plane.

use anyhow::Result;

use super::action::Action;

pub(super) async fn run(action: Action) -> Result<String> {
    match action {
        Action::Help => Ok(super::format::HELP.into()),
        Action::List => Ok(super::format::sessions(
            codetether_mux_control::list_sessions().await?,
        )),
        Action::New {
            name,
            workspace,
            no_worktree,
        } => {
            let isolation = codetether_mux_workspace::Isolation::from_no_worktree(no_worktree);
            let state =
                codetether_mux_lifecycle::start_session(&name, workspace, isolation).await?;
            Ok(super::format::changed("Started", &state))
        }
        Action::Window { name, workspace } => {
            let state = codetether_mux_control::create_window(&name, workspace).await?;
            Ok(super::format::changed("Added window to", &state))
        }
        Action::Select { name, id } => {
            let state = codetether_mux_control::select_window(&name, id).await?;
            Ok(super::format::changed("Selected window in", &state))
        }
        Action::Close { name, id } => {
            let state = codetether_mux_control::close_window(&name, id).await?;
            Ok(super::format::changed("Closed window in", &state))
        }
        Action::Kill { name } => {
            codetether_mux_control::stop_session(&name).await?;
            Ok(format!("Stopped mux session `{name}`."))
        }
    }
}

//! Trailing agent and autochat command routing over frontend callbacks.

/// Declare trailing-command routing using the frontend's existing handlers.
///
/// The dispatcher type supplies application, workspace and session references.
/// Callbacks preserve the synchronous form/list/autochat operations and awaited
/// spawn/kill/agent operations.
///
/// # Examples
/// ```text
/// codetether_slash_command::declare_tail_commands!(
///     Dispatch; open_form, spawn_task, kill, list, agent, autochat,
/// );
/// ```
#[macro_export]
macro_rules! declare_tail_commands {
    ($dispatch:ident; $form:path, $spawn:path, $kill:path, $list:path,
     $agent:path, $autochat:path $(,)?) => {
        mod tail {
            use super::$dispatch;
            use $crate::pipeline::TailCommand;

            pub(super) async fn run(host: &mut $dispatch<'_>, kind: TailCommand, rest: &str) {
                let $dispatch {
                    app, cwd, session, ..
                } = host;
                match kind {
                    TailCommand::SpawnForm => $form(app),
                    TailCommand::SpawnTask => $spawn(app, session, cwd, rest).await,
                    TailCommand::Kill => $kill(app, session, rest).await,
                    TailCommand::Agents => $list(app),
                    TailCommand::Agent => $agent(app, session, rest).await,
                    TailCommand::Autochat => $autochat(app, rest),
                }
            }
        }
    };
}

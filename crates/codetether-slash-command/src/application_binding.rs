//! Frontend dispatcher state and callback selection.

/// Declare the shared slash dispatcher over existing frontend state and handlers.
///
/// Stage callbacks retain leading/simple/tail routing. Runtime callbacks retain
/// Ralph/swarm behavior; session callbacks retain forage, goal, A2A, undo, detach
/// and fork operations. The host only forwards values and updates unknown status.
/// # Examples
/// ```text
/// codetether_slash_command::declare_dispatch_host!(
///     App, Session, Registry; leading, simple, tail; ralph, swarm;
///     forage, goal, a2a, undo, detach, fork,
/// );
/// ```
#[macro_export]
macro_rules! declare_dispatch_host {
    ($app:ty, $session:ty, $registry:ty;
     $leading:path, $simple:path, $tail:path;
     $ralph:path, $swarm:path;
     $forage:path, $goal:path, $a2a:path, $undo:path, $detach:path, $fork:path $(,)?) => {
        pub(super) struct Dispatch<'a> {
            pub app: &'a mut $app,
            pub cwd: &'a ::std::path::Path,
            pub session: &'a mut $session,
            pub registry: Option<&'a ::std::sync::Arc<$registry>>,
        }
        impl $crate::pipeline::Host for Dispatch<'_> {
            async fn leading(&mut self, kind: $crate::routing::LeadingCommand, rest: &str) {
                $leading(self, kind, rest).await;
            }
            async fn ralph(&mut self, rest: &str) -> bool {
                $ralph(self.app, self.cwd, self.session, self.registry, rest).await
            }
            fn swarm(&mut self, rest: &str) -> bool {
                $swarm(self.app, self.session, rest)
            }
            async fn session(&mut self, normalized: &str) -> bool {
                $crate::pipeline::session::dispatch(self, normalized).await
            }
            async fn simple(&mut self, kind: $crate::pipeline::SimpleCommand) {
                $simple(self, kind).await;
            }
            async fn tail(&mut self, kind: $crate::pipeline::TailCommand, rest: &str) {
                $tail(self, kind, rest).await;
            }
            fn unknown(&mut self, normalized: &str) {
                self.app.state.status = format!("Unknown command: {normalized}");
            }
        }
        impl $crate::pipeline::session::Host for Dispatch<'_> {
            fn forage(&mut self, rest: &str) {
                $forage(self.app, self.session, rest);
            }
            async fn goal(&mut self, rest: &str) {
                $goal(self.app, &self.session.id, rest).await;
            }
            fn a2a(&mut self, rest: &str) {
                $a2a(self.app, rest);
            }
            async fn undo(&mut self, rest: &str) {
                $undo(self.app, self.session, rest).await;
            }
            async fn detach(&mut self, rest: &str) {
                $detach(self.app, self.session, rest).await;
            }
            async fn fork(&mut self, rest: &str) {
                $fork(self.app, self.cwd, self.session, rest).await;
            }
        }
    };
}

/// Declare the public slash-command entry point in the frontend command module.
///
/// The containing module supplies `dispatch_host::Dispatch` with the normal
/// frontend state, workspace, session and optional provider registry fields.
#[macro_export]
macro_rules! declare_slash_entry {
    ($app:ty, $session:ty, $registry:ty $(,)?) => {
        /// Dispatch one slash command using the existing frontend session context.
        /// # Arguments
        /// Receives the frontend, workspace, session, optional registry and command text.
        /// # Returns
        /// Unit after the shared pipeline completes handling the command.
        /// # Examples
        /// ```text
        /// handle_slash_command(app, cwd, session, registry, command).await;
        /// ```
        pub async fn handle_slash_command(
            app: &mut $app,
            cwd: &::std::path::Path,
            session: &mut $session,
            registry: Option<&::std::sync::Arc<$registry>>,
            command: &str,
        ) {
            $crate::pipeline::dispatch(
                &mut dispatch_host::Dispatch {
                    app,
                    cwd,
                    session,
                    registry,
                },
                command,
            )
            .await;
        }
    };
}

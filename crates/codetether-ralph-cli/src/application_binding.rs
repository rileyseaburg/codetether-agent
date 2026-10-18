//! Frontend loop construction and monitor bindings for Ralph commands.

/// Declare Ralph command handling with existing runtime and frontend types.
///
/// Event delivery, loop execution, model fallback and monitor state retain
/// their existing owners. The notice callback uses normal frontend messages.
/// # Examples
/// ```text
/// codetether_ralph_cli::declare_frontend_ralph!(
///     App, Session, RalphLoop, Registry, ViewMode, push_notice,
/// );
/// ```
#[macro_export]
macro_rules! declare_frontend_ralph {
    ($app:ty, $session:ty, $run:ty, $registry:ty, $mode:path, $notice:path $(,)?) => {
        mod ralph_host {
            use ::std::{path::PathBuf, sync::Arc};
            use $crate::__private::{Provider, RalphConfig, RalphEvent, Result, mpsc};
            pub(super) struct AppHost;
            impl $crate::interactive::Host for AppHost {
                type Run = $run;
                async fn create(
                    path: PathBuf,
                    provider: Arc<dyn Provider>,
                    model: String,
                    config: RalphConfig,
                    events: mpsc::Sender<RalphEvent>,
                ) -> Result<$run> {
                    <$run>::new(path, provider, model, config)
                        .await
                        .map(|run| run.with_event_tx(events))
                }
                async fn run(run: &mut $run) -> Result<()> {
                    run.run().await.map(|_| ())
                }
            }
        }
        mod ralph_view {
            use $crate::__private::{RalphEvent, RalphStoryInfo, mpsc};
            use $mode as ViewMode;
            pub(super) struct CommandView<'a>(pub(super) &'a mut $app);
            impl $crate::interactive::View for CommandView<'_> {
                fn set_status(&mut self, status: String) {
                    self.0.state.status = status;
                }
                fn push_system_message(&mut self, message: String) {
                    $notice(self.0, message);
                }
                fn attach_event_rx(&mut self, events: mpsc::Receiver<RalphEvent>) {
                    self.0.state.ralph.attach_event_rx(events);
                }
                fn show_monitor(&mut self) {
                    self.0.state.set_view_mode(ViewMode::Ralph);
                }
                fn stories(&self) -> &[RalphStoryInfo] {
                    &self.0.state.ralph.stories
                }
                fn current_iteration(&self) -> usize {
                    self.0.state.ralph.current_iteration
                }
                fn max_iterations(&self) -> usize {
                    self.0.state.ralph.max_iterations
                }
            }
        }

        /// Dispatch a recognized Ralph subcommand through the shared policy.
        async fn handle_ralph_subcommand(
            app: &mut $app,
            cwd: &::std::path::Path,
            session: &$session,
            registry: Option<&::std::sync::Arc<$registry>>,
            rest: &str,
        ) -> bool {
            $crate::interactive::handle::<ralph_host::AppHost, $registry, _>(
                &mut ralph_view::CommandView(app),
                cwd,
                session.metadata.model.as_deref(),
                registry,
                rest,
            )
        }
    };
}

//! Frontend session and provider bindings for interactive goal preparation.

/// Declare the frontend `/go` command using existing application services.
///
/// Selects application/session types, the existing provider registry and model
/// resolver, system notices and the default agent count. Repository state and
/// pending approval remain in the frontend; shared drafting policy is unchanged.
/// # Examples
/// ```text
/// codetether_relay_okr::declare_go_command!(
///     App, Session, Registry, resolve_model, push_notice, DEFAULT_AGENTS,
/// );
/// ```
#[macro_export]
macro_rules! declare_go_command {
    ($app:ty, $session:ty, $registry:ty, $resolve:path, $notice:path, $agents:path $(,)?) => {
        mod go_host {
            use ::std::sync::Arc;
            use $crate::__private::{OkrRepository, Provider};
            use $crate::{
                PendingOkrApproval,
                interactive::{CommandHost, DraftHost},
            };
            struct Go<'a> {
                app: &'a mut $app,
                session: &'a mut $session,
            }
            impl CommandHost for Go<'_> {
                fn current_model(&self) -> Option<&str> {
                    self.session.metadata.model.as_deref()
                }
                fn set_model(&mut self, model: String) {
                    self.session.metadata.model = Some(model);
                }
                async fn save_session(&self) -> Result<(), String> {
                    self.session.save().await.map_err(|error| error.to_string())
                }
                fn repository(&mut self) -> &mut Option<Arc<OkrRepository>> {
                    &mut self.app.state.okr_repository
                }
                fn push_system_message(&mut self, message: String) {
                    $notice(self.app, message);
                }
                fn set_pending(&mut self, pending: PendingOkrApproval) {
                    self.app.state.pending_okr_approval = Some(pending);
                }
                fn set_status(&mut self, status: String) {
                    self.app.state.status = status;
                }
            }
            impl DraftHost for Go<'_> {
                type Registry = $registry;
                async fn load_registry() -> Option<Arc<$registry>> {
                    <$registry>::shared_from_vault().await.ok()
                }
                fn resolve(
                    registry: &Arc<$registry>,
                    model: &str,
                ) -> Option<(Arc<dyn Provider>, String)> {
                    $resolve(registry, model)
                }
            }
            pub(super) async fn execute(app: &mut $app, session: &mut $session, rest: &str) {
                $crate::interactive::execute(&mut Go { app, session }, rest, $agents).await;
            }
        }
        async fn handle_go_command(
            app: &mut $app,
            session: &mut $session,
            _registry: Option<&::std::sync::Arc<$registry>>,
            rest: &str,
        ) {
            go_host::execute(app, session, rest).await;
        }
    };
}

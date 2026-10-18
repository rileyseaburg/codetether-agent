//! Application preference state and setter selection for settings commands.

/// Declare preference command handling with the application's existing setters.
///
/// State reads and status updates use the existing frontend fields. The three
/// setter callbacks retain their persistence and runtime effects; declaring
/// the adapter does not invoke them or change any setting.
/// # Examples
/// ```text
/// codetether_settings_policy::declare_setting_commands!(
///     App, Session, set_auto_apply, set_network, set_autocomplete,
/// );
/// ```
#[macro_export]
macro_rules! declare_setting_commands {
    ($app:ty, $session:ty, $auto:path, $network:path, $autocomplete:path $(,)?) => {
        mod settings_command {
            use $crate::command::{Host, Setting};
            struct Settings<'a> {
                app: &'a mut $app,
                session: &'a mut $session,
            }
            impl Host for Settings<'_> {
                fn current(&self, setting: Setting) -> bool {
                    match setting {
                        Setting::AutoApply => self.app.state.auto_apply_edits,
                        Setting::Network => self.app.state.allow_network,
                        Setting::Autocomplete => self.app.state.slash_autocomplete,
                    }
                }
                async fn apply(&mut self, setting: Setting, enabled: bool) {
                    match setting {
                        Setting::AutoApply => $auto(self.app, self.session, enabled).await,
                        Setting::Network => $network(self.app, self.session, enabled).await,
                        Setting::Autocomplete => {
                            $autocomplete(self.app, self.session, enabled).await
                        }
                    }
                }
                fn set_status(&mut self, status: String) {
                    self.app.state.status = status;
                }
            }
            pub(super) async fn execute(
                app: &mut $app,
                session: &mut $session,
                setting: Setting,
                rest: &str,
            ) {
                $crate::command::execute(&mut Settings { app, session }, setting, rest).await;
            }
        }
    };
}

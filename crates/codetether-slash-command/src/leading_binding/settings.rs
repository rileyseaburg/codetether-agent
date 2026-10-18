//! settings stage bindings for leading commands.
/// Supporting implementation for [crate::declare_leading_commands].
#[doc(hidden)]
#[macro_export]
macro_rules! __leading_settings {
    ($dispatch:ty, $kind:path, $settings:path) => {
        mod settings {
            use $crate::routing::LeadingCommand;
            use $kind as Setting;
            pub(super) async fn run(host: &mut $dispatch, kind: LeadingCommand, rest: &str) {
                let app = &mut *host.app;
                let session = &mut *host.session;
                match kind {
                    $crate::routing::LeadingCommand::AutoApply => {
                        $settings(app, session, Setting::AutoApply, rest).await;
                        return;
                    }
                    $crate::routing::LeadingCommand::Network => {
                        $settings(app, session, Setting::Network, rest).await;
                        return;
                    }
                    $crate::routing::LeadingCommand::Autocomplete => {
                        $settings(app, session, Setting::Autocomplete, rest).await;
                        return;
                    }
                    _ => unreachable!("command routed to the wrong native adapter"),
                }
            }
        }
    };
}

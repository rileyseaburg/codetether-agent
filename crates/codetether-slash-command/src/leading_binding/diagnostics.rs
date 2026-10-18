//! diagnostics stage bindings for leading commands.
/// Supporting implementation for [crate::declare_leading_commands].
#[doc(hidden)]
#[macro_export]
macro_rules! __leading_diagnostics {
    ($dispatch:ty, $context:path, $mcp:path) => {
        mod diagnostics {
            use $crate::routing::LeadingCommand;

            pub(super) async fn run(host: &mut $dispatch, kind: LeadingCommand, rest: &str) {
                let app = &mut *host.app;
                match kind {
                    $crate::routing::LeadingCommand::Context => {
                        $context(app, rest);
                        return;
                    }
                    $crate::routing::LeadingCommand::Mcp => {
                        $mcp(app, rest).await;
                        return;
                    }
                    _ => unreachable!("command routed to the wrong native adapter"),
                }
            }
        }
    };
}

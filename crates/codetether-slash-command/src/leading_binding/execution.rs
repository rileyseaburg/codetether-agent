//! execution stage bindings for leading commands.
/// Supporting implementation for [crate::declare_leading_commands].
#[doc(hidden)]
#[macro_export]
macro_rules! __leading_execution {
    ($dispatch:ty, $go:path, $ask:path) => {
        mod execution {
            use $crate::routing::LeadingCommand;

            pub(super) async fn run(host: &mut $dispatch, kind: LeadingCommand, rest: &str) {
                let app = &mut *host.app;
                let session = &mut *host.session;
                let registry = host.registry;
                match kind {
                    $crate::routing::LeadingCommand::Go => {
                        $go(app, session, registry, rest).await;
                        return;
                    }
                    $crate::routing::LeadingCommand::Ask => {
                        $ask(app, session, registry, rest).await;
                        return;
                    }
                    _ => unreachable!("command routed to the wrong native adapter"),
                }
            }
        }
    };
}

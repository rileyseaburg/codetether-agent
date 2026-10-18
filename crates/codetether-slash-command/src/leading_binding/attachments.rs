//! attachments stage bindings for leading commands.
/// Supporting implementation for [crate::declare_leading_commands].
#[doc(hidden)]
#[macro_export]
macro_rules! __leading_attachments {
    ($dispatch:ty, $presentation:path, $image:path, $file:path, $edit:path) => {
        mod attachments {
            use $crate::routing::LeadingCommand;
            use $presentation as Presentation;
            pub(super) fn run(host: &mut $dispatch, kind: LeadingCommand, rest: &str) {
                let app = &mut *host.app;
                let cwd = host.cwd;
                match kind {
                    $crate::routing::LeadingCommand::Image => {
                        let cleaned = rest.trim().trim_matches(|c| c == '"' || c == '\'');
                        $image(app, cwd, cleaned, Presentation::Chat);
                        return;
                    }
                    $crate::routing::LeadingCommand::File => {
                        let cleaned = rest.trim().trim_matches(|c| c == '"' || c == '\'');
                        $file(app, cwd, cleaned);
                        return;
                    }
                    $crate::routing::LeadingCommand::Edit => {
                        $edit(
                            app,
                            cwd,
                            rest.trim().trim_matches(|c| c == '"' || c == '\''),
                        );
                        return;
                    }
                    _ => unreachable!("command routed to the wrong native adapter"),
                }
            }
        }
    };
}

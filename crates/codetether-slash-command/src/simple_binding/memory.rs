//! memory actions for simple frontend commands.
/// Supporting implementation for [crate::declare_simple_commands].
#[doc(hidden)]
#[macro_export]
macro_rules! __simple_memory {
    ($dispatch:ty, $recall:path) => {
        mod memory {
            use $crate::pipeline::SimpleCommand;

            pub(super) fn run(host: &mut $dispatch, kind: SimpleCommand) {
                match kind {
                    SimpleCommand::Recall => {
                        let cwd = std::path::PathBuf::from(&host.app.state.cwd_display);
                        $recall(host.app, &cwd);
                    }
                    _ => unreachable!("command routed to the wrong native adapter"),
                }
            }
        }
    };
}

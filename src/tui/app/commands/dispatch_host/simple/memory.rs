//! Native workspace binding for shared project recall policy.
mod host;
use super::*;
pub(super) fn run(host: &mut Dispatch<'_>, kind: SimpleCommand) {
    match kind {
        SimpleCommand::Recall => {
            let cwd = std::path::PathBuf::from(&host.app.state.cwd_display);
            codetether_cognition_core::palace::command::recall(host.app, &cwd);
        }
        _ => unreachable!("command routed to the wrong native adapter"),
    }
}

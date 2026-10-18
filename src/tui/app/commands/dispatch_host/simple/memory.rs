//! memory command execution bindings.
use super::*;
pub(super) fn run(host: &mut Dispatch<'_>, kind: SimpleCommand) {
    let app = &mut *host.app;
    match kind {
        SimpleCommand::Recall => {
            let beliefs =
                crate::memory::palace::load_project_beliefs(Path::new(&app.state.cwd_display));
            if beliefs.is_empty() {
                push_system_message(
                    app,
                    "No project memories found. Memories accumulate as you work.",
                );
            } else {
                let active: Vec<&crate::cognition::beliefs::Belief> = beliefs
                    .iter()
                    .filter(|b| b.confidence >= 0.5)
                    .take(30)
                    .collect();
                let mut lines = vec![format!(
                    "🏰 Project Memory Palace ({} beliefs, {} active)",
                    beliefs.len(),
                    active.len()
                )];
                for b in &active {
                    lines.push(format!("  • {} [conf: {:.2}]", b.claim, b.confidence));
                }
                push_system_message(app, lines.join("\n"));
            }
            app.state.status = "Project memory palace".to_string();
        }
        _ => unreachable!("command routed to the wrong native adapter"),
    }
}

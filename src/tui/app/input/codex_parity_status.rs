use std::path::Path;

use crate::config::{Config, TrustPolicyStatus};
use crate::session::Session;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

#[path = "codex_parity_status_lines.rs"]
mod lines;

pub(super) async fn show(app: &mut App, cwd: &Path, session: &Session, label: &str) {
    let policy = policy_line(cwd).await;
    let context = lines::context(app);
    let model = session.metadata.model.as_deref().unwrap_or("not selected");
    let last = lines::completion(app);
    let text = format!(
        "{label}\nSession: {}\nWorkspace: {}\nModel: {model}\n{policy}\n{}\n{context}\n{last}",
        session.id,
        cwd.display(),
        lines::tui(app),
    );
    push(app, text);
    app.state.clear_input();
    app.state.status = format!("{label} shown");
}

async fn policy_line(cwd: &Path) -> String {
    match Config::load_for_workspace(cwd).await {
        Ok(config) => {
            let status = TrustPolicyStatus::from_config(&config);
            crate::tui::ui::trust_status::set_status(status);
            crate::tui::ui::trust_status::format_status(&status)
        }
        Err(error) => format!("Policy: unavailable ({error})"),
    }
}

fn push(app: &mut App, text: String) {
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, text));
    app.state.scroll_to_bottom();
}

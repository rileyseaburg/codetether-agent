//! Mux session listing across every workspace server.

use super::list_entry::ListedSession;
use anyhow::Result;

pub(super) async fn run(json: bool) -> Result<()> {
    let mut listed = Vec::new();
    for record in codetether_mux_registry::list().await? {
        let reachable = tokio::time::timeout(
            std::time::Duration::from_millis(300),
            codetether_mux_client::handshake::probe(&record),
        )
        .await
        .is_ok_and(|result| result.is_ok());
        listed.extend(record.state.sessions.iter().map(|session| ListedSession {
            name: session.name.clone(),
            workspace: record.state.workspace.display().to_string(),
            address: record.address.to_string(),
            pid: record.pid,
            windows: session.windows.len(),
            active: session.active_window,
            reachable,
        }));
    }
    if json {
        println!("{}", serde_json::to_string_pretty(&listed)?);
    } else if listed.is_empty() {
        println!("no mux sessions");
    } else {
        for item in listed {
            super::list_render::print_item(&item);
        }
    }
    Ok(())
}

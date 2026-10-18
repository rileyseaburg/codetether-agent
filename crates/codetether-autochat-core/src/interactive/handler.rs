//! Event loop handler for autochat UI events.

use super::host::View;

use super::events::AutochatUiEvent;

/// Process a single autochat UI event, updating app state.
pub fn handle_autochat_event<E>(app: &mut impl View<E>, event: AutochatUiEvent<E>) {
    match event {
        AutochatUiEvent::Progress(msg) => {
            app.set_status(msg);
        }
        AutochatUiEvent::SystemMessage(msg) => {
            app.push_system(msg);
            app.scroll_to_bottom();
        }
        AutochatUiEvent::AgentEvent {
            agent_name,
            event: _,
        } => {
            app.set_status(format!("🔄 {agent_name} processing…"));
        }
        AutochatUiEvent::Completed {
            summary,
            okr_id,
            okr_run_id,
            relay_id,
        } => {
            app.autochat().running = false;
            let mut msg = format!("✅ Autochat relay completed: {summary}");
            if let Some(oid) = okr_id {
                msg.push_str(&format!("\nOKR: {oid}"));
            }
            if let Some(rid) = relay_id {
                msg.push_str(&format!("\nRelay: {rid}"));
            }
            let _ = okr_run_id;
            app.push_system(msg);
            app.scroll_to_bottom();
        }
    }
}

//! Coalesced history upload without cloning the message graph.

use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

use crate::session::Session;
use crate::session::history_sink::HistorySinkConfig;

static IN_FLIGHT: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

pub(super) fn spawn(session: &Session, config: HistorySinkConfig) {
    let in_flight = IN_FLIGHT.get_or_init(|| Mutex::new(HashSet::new()));
    let inserted = in_flight
        .lock()
        .map(|mut sessions| sessions.insert(session.id.clone()))
        .unwrap_or(false);
    if !inserted {
        tracing::debug!(session_id = %session.id, "history upload already in flight");
        return;
    }
    let body = match crate::session::history_sink::encode_jsonl_delta(&session.messages, 0) {
        Ok(body) => body.into_bytes(),
        Err(error) => {
            release(&session.id);
            tracing::warn!(%error, session_id = %session.id, "history encoding failed");
            return;
        }
    };
    let session_id = session.id.clone();
    tokio::spawn(async move {
        if let Err(error) =
            crate::session::history_sink::upload_encoded_history(&config, &session_id, body).await
        {
            tracing::warn!(%error, %session_id, "history sink upload failed (non-fatal)");
        }
        release(&session_id);
    });
}

fn release(session_id: &str) {
    if let Some(in_flight) = IN_FLIGHT.get()
        && let Ok(mut sessions) = in_flight.lock()
    {
        sessions.remove(session_id);
    }
}

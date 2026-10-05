//! Serialize only the changed message suffix and its page classifications.
use crate::session::Session;
use anyhow::Result;
pub(super) fn messages(session: &Session, from: usize) -> Result<Vec<String>> {
    session.messages[from..]
        .iter()
        .enumerate()
        .map(|(i, message)| {
            let page = session
                .pages
                .get(from + i)
                .copied()
                .unwrap_or_else(|| crate::session::pages::classify(message));
            Ok(serde_json::to_string(&(message, page))?)
        })
        .collect()
}

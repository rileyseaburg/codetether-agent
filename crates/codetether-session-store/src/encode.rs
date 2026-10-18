//! Serialize only the changed message suffix and its page classifications.
use crate::Snapshot;
use anyhow::Result;
pub(super) fn messages<S: Snapshot>(session: &S, from: usize) -> Result<Vec<String>> {
    session.messages()[from..]
        .iter()
        .enumerate()
        .map(|(i, message)| {
            let page = session
                .pages()
                .get(from + i)
                .copied()
                .unwrap_or_else(|| codetether_context_policy::pages::classify(message));
            Ok(serde_json::to_string(&(message, page))?)
        })
        .collect()
}

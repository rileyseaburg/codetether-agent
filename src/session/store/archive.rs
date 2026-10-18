//! Native session binding for the shared incremental history archive.
use crate::session::Session;
mod host;
pub(crate) fn schedule(session: &Session) {
    let Some(config) = session.metadata.history_sink.clone() else {
        return;
    };
    let id = session.id.clone();
    codetether_history_sink::archive::schedule::<host::ApplicationArchive>(id, config);
}

//! Acknowledge mutation identities only after a successful write or consistent read.
use crate::session::Session;
pub(super) fn clean(session: &Session) {
    session.storage.0.lock().unwrap().versions = [
        session.messages.version(),
        session.pages.version(),
        session.tool_uses.version(),
    ];
    session.messages.clean();
    session.pages.clean();
    session.tool_uses.clean();
}

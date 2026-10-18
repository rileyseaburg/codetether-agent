//! Native host binding for shared automatic recall injection.
use crate::{provider::Message, session::Session};
pub(crate) async fn message(session: &Session) -> Option<Message> {
    codetether_recall_context::message(session, &super::storage_host::NativeHost).await
}

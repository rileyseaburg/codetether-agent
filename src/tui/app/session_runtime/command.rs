//! Native specializations of shared owned runtime request and command types.
use crate::{
    provider::ProviderRegistry,
    session::{ImageAttachment, Session},
};
pub(crate) type PromptRequest =
    codetether_session_runtime::request::PromptRequest<Session, ProviderRegistry, ImageAttachment>;
pub(crate) type SessionCommand = codetether_session_runtime::request::SessionCommand<PromptRequest>;
impl codetether_session_runtime::request::SessionIdentity for Session {
    fn session_id(&self) -> &str {
        &self.id
    }
}

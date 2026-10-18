//! Native specializations of shared owned runtime request and command types.
use crate::{
    provider::ProviderRegistry,
    session::{ImageAttachment, Session},
};
pub(crate) type PromptRequest =
    codetether_session_runtime::request::PromptRequest<Session, ProviderRegistry, ImageAttachment>;
codetether_session_runtime::impl_frontend_session_identity!(Session);
#[cfg(test)]
pub(crate) type SessionCommand = codetether_session_runtime::request::SessionCommand<PromptRequest>;

//! Client construction supplied by a manager's host application.
use crate::{LspClient, LspConfig, client_cache::CacheClient};
use anyhow::Result;
use std::future::Future;
use std::ops::Deref;
/// A cached client wrapper that supplies its own initialization identity.
///
/// # Implementors
/// Applications implement this for wrappers dereferencing to [LspClient].
/// Construction and initialization futures must be safe to send across tasks.
pub trait ManagedClient: CacheClient + Deref<Target = LspClient> + Sized {
    /// Start a server with explicit configuration.
    ///
    /// # Errors
    /// The returned future reports server discovery or process startup errors.
    fn new(config: LspConfig) -> impl Future<Output = Result<Self>> + Send;
    /// Start the built-in server for a language and root URI.
    ///
    /// # Errors
    /// The returned future reports unknown languages or server startup errors.
    fn for_language(
        language: &str,
        root_uri: Option<String>,
    ) -> impl Future<Output = Result<Self>> + Send;
    /// Initialize the server with the wrapper's client identity.
    ///
    /// # Errors
    /// The returned future reports initialization, transport or decoding errors.
    fn initialize(&self) -> impl Future<Output = Result<()>> + Send;
}

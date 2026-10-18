//! Copilot provider construction with a caller-owned application identity.
//!
//! [CopilotProvider] delegates protocol operations to the shared provider while
//! [Identity] supplies the application's User-Agent. This keeps version selection
//! with the application instead of substituting this library's package version.
use std::marker::PhantomData;
mod construct;
mod delegate;
/// Source of the application identity sent with Copilot requests.
/// # Implementors
/// Applications return their existing User-Agent string.
pub trait Identity {
    /// Return the User-Agent for newly constructed providers.
    fn user_agent() -> String;
}
/// Copilot provider carrying the caller's application identity.
/// # Examples
/// ```no_run
/// use codetether_provider_copilot::application::{CopilotProvider, Identity};
/// struct App;
/// impl Identity for App { fn user_agent() -> String { "example/1.0".into() } }
/// let _provider = CopilotProvider::<App>::new("token".into())?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct CopilotProvider<I: Identity>(crate::CopilotProvider, PhantomData<fn() -> I>);

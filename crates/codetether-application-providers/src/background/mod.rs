//! Background RLM routing with configured providers and typed session events.
//!
//! [`Background`] binds an application's provider policy and session snapshot
//! type. Its methods schedule context/tool summaries through the shared worker;
//! [`Notify`] forwards worker progress through the caller's session event sink.
use crate::Application;
use std::{marker::PhantomData, sync::Arc};
mod context;
mod notify;
mod resolve;
mod route;

/// Receives background progress wrapped in a session event.
pub type Notify<S> = Arc<dyn Fn(codetether_session_events::SessionEvent<S>) + Send + Sync>;

/// Stateless background worker binding for provider policy `H` and session `S`.
///
/// # Examples
/// ```no_run
/// # fn example<H: codetether_application_providers::Application>() {
/// let resolver: codetether_rlm_background::Resolver =
///     codetether_application_providers::background::Background::<H, ()>::resolve_model;
/// # }
/// ```
pub struct Background<H: Application, S>(PhantomData<fn() -> (H, S)>);

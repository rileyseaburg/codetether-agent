//! Standard cognition runtime assembly using application-selected services.
//!
//! [`ApplicationHost`] connects the shared thinker client and tool registry to
//! environment configuration, state restoration and persisted ticks. Its
//! `EMPTY_STATE` parameter preserves caller-selected test isolation. Native
//! backends and tool implementations remain selected by `H` and `R`.
use codetether_cognition_core::thinker::client::Host as ClientHost;
use std::marker::PhantomData;
mod implementation;
mod initialization;
mod tick;
pub use tick::TickServices;

/// Runtime services with an optional empty initial state for isolated callers.
///
/// `EMPTY_STATE = false` restores persisted cognition state; `true` starts empty.
/// It does not disable persistence during ticks.
/// # Examples
/// ```no_run
/// # fn example<H, R>() where H: codetether_cognition_core::thinker::client::Host + 'static, R: codetether_cognition_core::execution::dispatch::Registry + std::fmt::Debug + 'static {
/// let runtime = codetether_cognition_runtime::CognitionRuntime::<
///     codetether_cognition_runtime::application::ApplicationHost<H, R, false>
/// >::new_with_options(Default::default());
/// assert!(!runtime.is_enabled());
/// # }
/// ```
pub struct ApplicationHost<H: ClientHost, R, const EMPTY_STATE: bool>(PhantomData<fn() -> (H, R)>);

//! RLM tool routing with the application's event-bus type.
//!
//! [`ApplicationHost`] builds the existing tool-specific processing context and
//! calls the shared router. No event bus is attached to this tool context.
use std::marker::PhantomData;
mod context;
mod host;
/// RLM tool host preserving an application's routing bus specialization.
/// # Examples
/// ```no_run
/// # fn example<B: codetether_rlm_routing::Bus>() {
/// fn accepts_host<H: codetether_rlm_tool::Host>() {}
/// accepts_host::<codetether_rlm_tool::application::ApplicationHost<B>>();
/// # }
/// ```
pub struct ApplicationHost<B>(PhantomData<fn() -> B>);

//! Application-bound Ralph construction for approved-task execution.
//!
//! [`ApplicationHost`] implements the pipeline host for a provider registry and
//! its compatible shared [`codetether_ralph_runtime::RalphLoop`].
use std::marker::PhantomData;
mod host;
/// Pipeline host binding the existing provider registry and Ralph loop type.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_ralph_runtime::Host>() where H::Registry: codetether_go_prd::Registry {
/// type Bound<P, R> = codetether_go_ralph::application::ApplicationHost<P, R>;
/// fn accepts_host<T: codetether_go_ralph::Host>() {}
/// accepts_host::<Bound<H::Registry, codetether_ralph_runtime::RalphLoop<H>>>();
/// # }
/// ```
pub struct ApplicationHost<P, R>(PhantomData<fn() -> (P, R)>);

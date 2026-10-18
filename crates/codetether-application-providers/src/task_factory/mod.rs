//! Provider constructors paired with the owning application's cached fallback.
//!
//! [`TaskFactory`] forwards construction to [`crate::Application::Factory`]
//! and supplies the same shared Vault registry for task-key fallback.
use std::marker::PhantomData;
mod constructors;
mod platform;

/// Task provider factory retaining the application's provider cache identity.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_application_providers::Application>() -> anyhow::Result<()> {
/// let _registry = codetether_provider_registry::task::load_task_provider_registry::<
///     codetether_application_providers::task_factory::TaskFactory<H>,
/// >(None).await?;
/// # Ok(()) }
/// ```
pub struct TaskFactory<H>(PhantomData<fn() -> H>);

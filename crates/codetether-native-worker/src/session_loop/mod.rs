//! Scoped worker steps over native context, transcript and control services.
//!
//! [`run`] preserves the shared driver's step limits and execution ownership.
//! [`Session`] combines the existing context/transcript contracts; [`Responses`]
//! supplies only the application-specific response and tool execution step.
use codetether_a2a_worker_core::session_protocol::{
    SessionStepsOutcome,
    context::{Context, ResponseContext},
};
use codetether_native_providers::CompletionResponse;
use std::{future::Future, marker::PhantomData};
mod host;

/// Session services required by the worker loop.
///
/// # Implementors
/// Any type already implementing context requests, session orchestration and
/// the worker transcript contract receives this implementation automatically.
pub trait Session:
    codetether_native_context::ContextHost
    + codetether_session_orchestration::Session
    + codetether_a2a_worker_core::session_protocol::transcript::Host
{
}
impl<S> Session for S where
    S: codetether_native_context::ContextHost
        + codetether_session_orchestration::Session
        + codetether_a2a_worker_core::session_protocol::transcript::Host
{
}

/// Application response handling for a session and tool registry.
///
/// # Implementors
/// Implementations record and execute the provider response using the
/// application's concrete session/tool services.
pub trait Responses<S, R> {
    /// Process one completed model response and report whether the step ended.
    /// # Errors
    /// Propagates the existing response/tool-processing errors.
    fn process(
        context: &ResponseContext<'_, R>,
        session: &mut S,
        output: &mut String,
        response: CompletionResponse,
    ) -> impl Future<Output = anyhow::Result<bool>> + Send;
}

struct Native<'a, 'context, S, R, P> {
    context: &'a Context<'context, R>,
    session: &'a mut S,
    responses: PhantomData<fn() -> P>,
}

/// Run the worker's scoped session driver with the supplied response handler.
/// # Arguments
/// * `context` - Existing provider, tool and response settings.
/// * `session` - Session whose control, transcript and persistence are updated.
/// * `max_steps` - Existing per-run step limit.
/// # Returns
/// The shared driver's final session outcome.
/// # Errors
/// Propagates completion, response, persistence or execution-control errors.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_native_worker::session_loop::Session, R: Sync, P: codetether_native_worker::session_loop::Responses<S, R>>(context: &codetether_a2a_worker_core::session_protocol::context::Context<'_, R>, session: &mut S) -> anyhow::Result<()> {
/// let _outcome = codetether_native_worker::session_loop::run::<S, R, P>(context, session, 10).await?;
/// # Ok(()) }
/// ```
pub async fn run<S: Session, R: Sync, P: Responses<S, R>>(
    context: &Context<'_, R>,
    session: &mut S,
    max_steps: usize,
) -> anyhow::Result<SessionStepsOutcome> {
    codetether_a2a_worker_core::session_protocol::driver::run_scoped(
        &mut Native::<_, _, P> {
            context,
            session,
            responses: PhantomData,
        },
        max_steps,
    )
    .await
}

//! Worker response sequencing with native telemetry and session tool hooks.
//!
//! [`Handler`] implements the session loop's response callback. [`Tools`]
//! supplies application tool-input enrichment and result recording; native
//! control checks, usage accounting and transcript append remain here.
use codetether_a2a_worker_core::session_protocol::{
    ToolCall, context::ResponseContext, transcript,
};
use codetether_native_context::ContextHost;
use codetether_native_providers::CompletionResponse;
use std::{future::Future, marker::PhantomData};
mod host;

/// Application-specific tool-call effects used by the response driver.
/// # Implementors
/// Implementations preserve the session's input enrichment and result recording.
pub trait Tools<S, R> {
    /// Execute a tool call using the original response context and session.
    fn execute(
        session: &mut S,
        context: &ResponseContext<'_, R>,
        tool_call: ToolCall,
    ) -> impl Future<Output = ()> + Send;
}

/// Session-loop response callback using tool effects `E`.
///
/// # Examples
/// Select `Handler<E>` as the response parameter of [`crate::session_loop::run`].
/// ```no_run
/// type Bound<E> = codetether_native_worker::response::Handler<E>;
/// # let _: Option<Bound<()>> = None;
/// ```
pub struct Handler<E>(PhantomData<fn() -> E>);

struct Native<'a, 'context, S, R, E> {
    context: &'a ResponseContext<'context, R>,
    session: &'a mut S,
    tools: PhantomData<fn() -> E>,
}

impl<S, R, E> crate::session_loop::Responses<S, R> for Handler<E>
where
    S: ContextHost + transcript::Host,
    R: Sync,
    E: Tools<S, R>,
{
    async fn process(
        context: &ResponseContext<'_, R>,
        session: &mut S,
        output: &mut String,
        response: CompletionResponse,
    ) -> anyhow::Result<bool> {
        codetether_a2a_worker_core::session_protocol::response::process_response(
            &mut Native::<_, _, E> {
                context,
                session,
                tools: PhantomData,
            },
            context.model,
            &context.output_callback,
            output,
            response,
        )
        .await
    }
}

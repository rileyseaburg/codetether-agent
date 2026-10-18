//! Register, supervise and stop the bridge's child tasks.
use crate::{Registration, RegistrationRequest, context::Context};
pub(crate) async fn run<R: Registration>(ctx: Context) {
    let token_opt = Some(ctx.identity.token.clone());
    if let Err(e) = R::register(RegistrationRequest {
        client: &ctx.client,
        server: &ctx.identity.server,
        token: &token_opt,
        worker_id: &ctx.identity.worker_id,
        worker_name: &ctx.identity.worker_name,
        codebases: &ctx.codebases,
    })
    .await
    {
        tracing::warn!("Failed to register worker with A2A server: {}", e);
    }
    let heartbeat_handle = crate::heartbeat::start(&ctx, token_opt);
    let sse_handle = tokio::spawn(crate::stream::run(ctx.identity.clone(), ctx.task_tx));
    crate::control::run(
        &ctx.identity,
        &ctx.client,
        &ctx.heartbeat_state,
        ctx.bus,
        ctx.cmd_rx,
    )
    .await;
    heartbeat_handle.abort();
    sse_handle.abort();
    tracing::info!("Worker bridge stopped");
}

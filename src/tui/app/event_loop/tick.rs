use super::ctx::SelectCtx;

pub async fn run(ctx: &mut SelectCtx<'_>) {
    super::autochat::drain_autochat(ctx.app);
    crate::tui::app::event_handlers::drain_voice_transcription(&mut ctx.app.state);
    super::inbox_tick::run(
        ctx.app,
        ctx.cwd,
        ctx.session,
        ctx.registry,
        ctx.worker_bridge,
        ctx.event_tx,
        ctx.result_tx,
    )
    .await;
    refresh_audit(ctx).await;
    ctx.app.state.ralph.drain_events();
    ctx.app.state.swarm.drain_events();
}

async fn refresh_audit(ctx: &mut SelectCtx<'_>) {
    if ctx.app.state.view_mode == crate::tui::models::ViewMode::Audit {
        crate::tui::audit_view::refresh_audit_snapshot(&mut ctx.app.state.audit).await;
    }
}

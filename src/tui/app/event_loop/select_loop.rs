//! Native state capture and waiting around shared event dispatch.
#[path = "dirty.rs"]
mod dirty;
#[path = "select_loop/host/mod.rs"]
mod host;
#[path = "queued_prompt.rs"]
mod queued_prompt;
#[path = "select_loop/wait.rs"]
mod wait;
pub(super) async fn select_once(args: &mut super::SelectArgs<'_>) -> anyhow::Result<bool> {
    let before = dirty::capture(args.app);
    let event = wait::next(args).await;
    codetether_tui_reactor::dispatch::dispatch(&mut host::Native(args), event, before).await
}

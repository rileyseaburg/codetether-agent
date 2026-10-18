//! Inject application clipboard access into the shared attachment flow.
pub(super) async fn run(
    target: &codetether_mux_registry::SessionTarget,
    host: &impl crate::Host,
) -> anyhow::Result<()> {
    codetether_mux_attachment::attach_session(target, || host.clipboard_content()).await
}

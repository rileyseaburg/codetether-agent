//! Preserve the application's canonical worker registration metadata.
pub(super) struct NativeRegistration;
#[async_trait::async_trait]
impl codetether_tui_worker::Registration for NativeRegistration {
    async fn register(
        request: codetether_tui_worker::RegistrationRequest<'_>,
    ) -> anyhow::Result<()> {
        crate::a2a::worker::register_worker(
            request.client,
            request.server,
            request.token,
            request.worker_id,
            request.worker_name,
            request.codebases,
            None,
        )
        .await
    }
}

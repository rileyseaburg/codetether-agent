//! Local CUDA runtime operations.
use super::*;
impl<R: Runtime> LocalCudaProvider<R> {
    pub(super) async fn runtime(&self, request: &CompletionRequest) -> Result<&R> {
        // Runtime config is captured on first use; changing env vars later
        // requires creating a new provider instance.
        self.runtime
            .get_or_try_init(|| async { R::new(self.build_config(request)?) })
            .await
    }
}

//! Resolve the test model and record when resolution happens.

use super::*;
#[async_trait::async_trait]
impl EmbeddingResolver for Resolver {
    async fn resolve(
        &self,
        selector: &str,
    ) -> anyhow::Result<(Arc<dyn EmbeddingProvider>, String)> {
        assert_eq!(selector, "remote/model");
        self.calls.resolutions.fetch_add(1, Ordering::SeqCst);
        Ok((
            Arc::new(Provider {
                calls: Arc::clone(&self.calls),
                failures: self.failures,
                dimensions: self.dimensions,
            }),
            "resolved-model".into(),
        ))
    }
}

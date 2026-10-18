//! Native materialized-view lookup for proactive context orchestration.
pub use anyhow;
pub use async_trait::async_trait;
pub use serde_json::Value;
/// Bind the existing projection reader to a proactive worker host.
/// # Usage
/// ```text
/// codetether_proactive_context::impl_projection_host!(Host, read_projection);
/// ```
#[macro_export]
macro_rules! impl_projection_host {
    ($host:ty, $read:path) => {
        #[$crate::host_binding::async_trait]
        impl $crate::Host for $host {
            async fn projection(
                &self,
                id: &str,
                name: &str,
            ) -> $crate::host_binding::anyhow::Result<Option<$crate::host_binding::Value>> {
                $read(id, name).await
            }
        }
    };
}

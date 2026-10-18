//! Native session paths and projection reads for recall sidecar storage.
pub use anyhow;
pub use async_trait::async_trait;
pub use serde_json::Value;
/// Declare a stateless recall storage host with the existing session paths.
/// # Usage
/// ```text
/// codetether_recall_store::declare_session_host!(pub struct Host; Session, read_projection);
/// ```
#[macro_export]
macro_rules! declare_session_host {
    ($visibility:vis struct $host:ident; $(,)? $session:ty, $projection:path) => {
        #[derive(Default)]
        $visibility struct $host;
        #[$crate::session_binding::async_trait]
impl $crate::Host for $host {
    fn session_path(&self, id: &str) -> $crate::session_binding::anyhow::Result<::std::path::PathBuf> {
        <$session>::session_path(id)
    }
    fn root(&self) -> $crate::session_binding::anyhow::Result<::std::path::PathBuf> {
        Ok(<$session>::sessions_dir()?.join("recall"))
    }
    async fn projection(&self, id: &str) -> $crate::session_binding::anyhow::Result<::std::option::Option<$crate::session_binding::Value>> {
        $projection(id, "recall").await
    }
}

    };
}

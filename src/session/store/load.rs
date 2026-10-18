//! Concrete session specialization of native indexed loading.
use crate::session::{Session, tail_load::TailLoad};
pub(crate) async fn load(path: &std::path::Path, window: usize) -> anyhow::Result<TailLoad> {
    codetether_native_session_store::load::<Session>(path, window).await
}

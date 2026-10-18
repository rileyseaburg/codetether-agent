//! Native session specialization of shared indexed loading.
use crate::session::{Session, tail_load::TailLoad};
use anyhow::Result;
use std::path::Path;
pub(crate) async fn load(path: &Path, window: usize) -> Result<TailLoad> {
    codetether_session_store::load::<Session>(path, window, super::migration::ensure).await
}

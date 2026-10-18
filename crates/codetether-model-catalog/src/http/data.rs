//! Registry loading and timestamped model collection for HTTP requests.
use super::{Host, ModelsResult, error};
use crate::discovery::{collect_models, types};

pub(super) async fn model_data<H: Host>() -> ModelsResult<Vec<types::Model>> {
    let registry = H::load_registry().await.map_err(error::load_error)?;
    Ok(collect_models(registry.as_ref(), chrono::Utc::now().timestamp()).await)
}

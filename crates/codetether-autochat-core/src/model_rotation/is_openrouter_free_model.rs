//! Auto-chat is openrouter free model.
use super::*;

pub(super) fn is_openrouter_free_model(model_id: &str) -> bool {
    let id = model_id.to_ascii_lowercase();
    id.contains(":free") || id.ends_with("-free")
}

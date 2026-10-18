//! Convert internal provider model metadata to API models.

use super::{ids, types};

type ModelInfo = codetether_provider_core::ModelInfo;

/// Project one provider model into the compatible discovery schema.
/// # Arguments
/// * `provider` - Provider name used in identifiers and display labels.
/// * `model` - Provider capability and pricing metadata.
/// * `created` - Timestamp for the record.
/// # Returns
/// A model entry retaining capabilities, prices and supported options.
/// # Examples
/// ```
/// # fn example(model: codetether_provider_core::ModelInfo) {
/// let _ = codetether_model_catalog::discovery::convert::convert_model("provider", model, 0);
/// # }
/// ```
pub fn convert_model(provider: &str, model: ModelInfo, created: i64) -> types::Model {
    let id = ids::model_id(provider, &model.id);
    let options = super::options::options(provider, &model.id);
    let supported_parameters = super::parameters::with_options(&model, &options);
    types::Model {
        id: id.clone(),
        canonical_slug: id,
        name: ids::display_name(provider, &model.name),
        created,
        description: format!(
            "{} model served by CodeTether provider {provider}",
            model.name
        ),
        context_length: model.context_window,
        architecture: super::architecture::architecture(&model),
        pricing: super::pricing::price(&model),
        top_provider: super::top_provider::top_provider(&model),
        per_request_limits: None,
        supported_parameters,
        options,
    }
}

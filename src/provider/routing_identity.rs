//! Adapter model normalization for authoritative per-request identity.

pub(super) fn resolve(provider: &str, model: &str) -> String {
    use super::super::{bedrock::BedrockProvider, openai_codex::OpenAiCodexProvider};
    match provider {
        "bedrock" => BedrockProvider::resolve_model_id(model).to_owned(),
        "openai-codex" => OpenAiCodexProvider::resolved_model_identity(model),
        "cerebras" => super::super::openai::alias::normalize_model_id(provider, model).into_owned(),
        "glm5" => super::super::glm5::Glm5Provider::normalize_model(model),
        "vertex-glm" => super::super::vertex_glm::model_id::normalize(model),
        _ => model.to_owned(),
    }
}

#[cfg(test)]
#[path = "routing_identity/tests.rs"]
mod tests;

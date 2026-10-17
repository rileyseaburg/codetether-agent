//! Model selection shared by Gemini Web transport and harness identity.

use super::{GeminiWebProtocol, MODELS};

fn selected(model: &str) -> &'static (&'static str, &'static str, &'static str, usize) {
    MODELS
        .iter()
        .find(|(id, _, _, _)| *id == model)
        .unwrap_or(&MODELS[0])
}

impl GeminiWebProtocol {
    /// Return the known model ID, using the original fallback for unknown IDs.
    pub fn resolved_model(model: &str) -> String {
        selected(model).0.to_owned()
    }

    /// Return the request-header mode ID for the resolved model.
    pub fn mode_id(model: &str) -> &'static str {
        selected(model).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn routed_identity_matches_gemini_web_fallback_and_modes() {
        for (id, _, _, _) in MODELS {
            assert_eq!(GeminiWebProtocol::resolved_model(id), *id);
        }
        for model in ["unknown", ""] {
            assert_eq!(GeminiWebProtocol::resolved_model(model), MODELS[0].0);
        }
        for model in MODELS.iter().map(|entry| entry.0).chain(["unknown", ""]) {
            let resolved = GeminiWebProtocol::resolved_model(model);
            assert_eq!(
                GeminiWebProtocol::mode_id(model),
                GeminiWebProtocol::mode_id(&resolved)
            );
        }
    }
}

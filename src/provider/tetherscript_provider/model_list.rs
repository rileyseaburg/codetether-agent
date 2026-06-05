//! Model-list normalization for TetherScript provider hooks.

use anyhow::Result;
use serde_json::Value;

use crate::provider::ModelInfo;

use super::runner::TetherScriptProvider;

impl TetherScriptProvider {
    pub(crate) async fn available_models(&self) -> Result<Vec<ModelInfo>> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.call_list_models()).await?
    }
}

pub(crate) fn parse_models(mut value: Value, provider: &str) -> Result<Vec<ModelInfo>> {
    add_missing_provider(&mut value, provider);
    serde_json::from_value(value).map_err(Into::into)
}

fn add_missing_provider(value: &mut Value, provider: &str) {
    let Some(models) = value.as_array_mut() else {
        return;
    };
    for model in models.iter_mut().filter(|model| model.is_object()) {
        if model.get("provider").is_none() {
            model["provider"] = Value::String(provider.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_models;

    #[test]
    fn fills_missing_provider_without_overwriting_existing_values() {
        let models = parse_models(
            serde_json::json!([
                super::super::model_record::without_provider("llama-4"),
                super::super::model_record::with_provider("other", "external")
            ]),
            "cerebras",
        )
        .expect("models should parse");

        assert_eq!(models[0].provider, "cerebras");
        assert_eq!(models[1].provider, "external");
    }
}

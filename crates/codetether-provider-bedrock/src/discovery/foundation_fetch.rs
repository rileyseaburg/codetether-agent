//! Fetch foundation model metadata.
use super::*;
impl BedrockClient<'_> {
    pub(super) async fn discover_foundation_models(&self, models: &mut HashMap<String, ModelInfo>) {
        let fm_url = format!("{}/foundation-models", self.management_url());
        let Ok(resp) = self.send_request("GET", &fm_url, None, "bedrock").await else {
            return;
        };
        if !resp.status().is_success() {
            return;
        }
        let Ok(data) = resp.json::<Value>().await else {
            return;
        };
        let Some(summaries) = data.get("modelSummaries").and_then(|v| v.as_array()) else {
            return;
        };

        for m in summaries {
            if let Some(info) = foundation_model_to_info(m) {
                models.insert(info.id.clone(), info);
            }
        }
    }
}

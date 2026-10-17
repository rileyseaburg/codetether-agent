//! Fetch profile model metadata.
use super::*;
impl BedrockClient<'_> {
    pub(super) async fn discover_inference_profiles(
        &self,
        models: &mut HashMap<String, ModelInfo>,
    ) {
        let ip_url = format!(
            "{}/inference-profiles?typeEquals=SYSTEM_DEFINED&maxResults=200",
            self.management_url()
        );
        let Ok(resp) = self.send_request("GET", &ip_url, None, "bedrock").await else {
            return;
        };
        if !resp.status().is_success() {
            return;
        }
        let Ok(data) = resp.json::<Value>().await else {
            return;
        };
        let Some(profiles) = data
            .get("inferenceProfileSummaries")
            .and_then(|v| v.as_array())
        else {
            return;
        };

        for p in profiles {
            if let Some(info) = inference_profile_to_info(p, models) {
                models.insert(info.id.clone(), info);
            }
        }
    }
}

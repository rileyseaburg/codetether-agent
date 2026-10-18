//! Human-readable model refresh outcome.
use crate::ModelRefreshSummary;

fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}
impl ModelRefreshSummary {
    fn skipped_providers(&self) -> usize {
        self.failed_providers + self.timed_out_providers
    }

    pub fn status_message(&self) -> String {
        if self.loaded_models == 0 {
            let skipped = self.skipped_providers();
            if self.provider_count == 0 || skipped == 0 {
                "No models available".to_string()
            } else {
                format!(
                    "No models available ({} provider{} skipped)",
                    skipped,
                    plural(skipped)
                )
            }
        } else {
            let mut status = format!(
                "Model picker: {} models from {} provider{}",
                self.loaded_models,
                self.loaded_providers,
                plural(self.loaded_providers)
            );
            let skipped = self.skipped_providers();
            if skipped > 0 {
                status.push_str(&format!(" ({} skipped)", skipped));
            }
            status
        }
    }
}

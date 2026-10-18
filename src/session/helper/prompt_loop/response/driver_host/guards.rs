//! Native retry-policy and model-view bindings for the response driver.
use super::super::super::Runner;
use crate::provider::CompletionResponse;
use codetether_prompt_loop::driver::{Guards, ProviderView};
impl Guards for Runner<'_> {
    fn provider_view(&self) -> ProviderView<'_> {
        ProviderView {
            provider: &self.model.provider_name,
            model: &self.model.model_id,
            tools: &self.model.tools,
        }
    }
    fn tool_first(&mut self, text: &str, calls: bool) -> anyhow::Result<bool> {
        super::super::build_guard::tool_first(self, text, calls)
    }
    fn native_call(
        &mut self,
        response: &CompletionResponse,
        text: &str,
        calls: bool,
    ) -> anyhow::Result<bool> {
        super::super::native_guard::native_call(self, response, text, calls)
    }
    fn thinking_only(&mut self, response: &CompletionResponse) -> bool {
        super::super::thinking_only::continue_instead(self, response)
    }
}

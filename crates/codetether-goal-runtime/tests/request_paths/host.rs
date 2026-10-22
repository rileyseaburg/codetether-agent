use codetether_context_request::attempt::{Host, Target};
use codetether_provider_core::{Message, ToolDefinition};

#[derive(Clone)]
pub struct Prepared {
    pub prompt: String,
    pub messages: Vec<Message>,
    pub extras: Vec<Message>,
    pub tools: Vec<ToolDefinition>,
    pub model: String,
    pub provider: String,
    pub refresh_to: Option<String>,
}
impl Host for Prepared {
    fn target(&self) -> Target<'_> {
        Target {
            system_prompt: &self.prompt,
            provider: &self.provider,
            model: &self.model,
            tools: &self.tools,
            temperature: Some(0.7),
        }
    }
    fn proactive(&self) -> Option<&Message> {
        None
    }
    async fn prepare(&mut self, extras: &mut Vec<Message>) {
        extras.extend(self.extras.clone());
        if let Some(prompt) = self.refresh_to.take() {
            self.prompt = prompt;
        }
    }
    fn history(&self) -> &[Message] {
        &self.messages
    }
}
impl codetether_context_request::Source for Prepared {
    fn session_id(&self) -> &str {
        "fixture"
    }
    fn effective_policy(&self, _: &str) -> codetether_context_policy::derivation::DerivePolicy {
        Default::default()
    }
    fn prepare(&self, _: std::sync::Arc<dyn codetether_provider_core::Provider>, _: &str) {}
}
#[async_trait::async_trait]
impl codetether_context_request::Host for Prepared {
    async fn derive(
        &self,
        _: codetether_context_request::Derivation<'_>,
    ) -> anyhow::Result<codetether_context_policy::projection::helpers::DerivedContext> {
        Ok(
            codetether_context_policy::projection::helpers::DerivedContext {
                messages: self.messages.clone(),
                origin_len: self.messages.len(),
                compressed: false,
                dropped_ranges: vec![],
                provenance: vec![],
                resolutions: vec![
                    codetether_context_policy::pages::ResidencyLevel::Full;
                    self.messages.len()
                ],
            },
        )
    }
    async fn prefetch(&self) -> Option<Message> {
        self.extras.first().cloned()
    }
}

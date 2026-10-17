impl OpenAiCodexProvider {
    fn available_models(&self) -> Vec<ModelInfo> {
        codetether_provider_core::codex::model_policy::available_models(
            self.using_chatgpt_backend(),
        )
    }
}

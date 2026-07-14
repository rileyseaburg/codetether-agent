impl OpenAiCodexProvider {
    fn model_is_supported_by_backend(&self, model: &str) -> bool {
        if !self.using_chatgpt_backend() {
            return true;
        }
        Self::chatgpt_supported_models().contains(&model)
    }
}

impl OpenAiCodexProvider {
    fn available_models(&self) -> Vec<ModelInfo> {
        let mut models = vec![
            Self::model_info("gpt-5.5", "GPT-5.5", 272_000, 128_000, false),
            Self::model_info("gpt-5.5-fast", "GPT-5.5 Fast", 272_000, 128_000, false),
            Self::model_info("gpt-5.6-sol", "GPT-5.6 Sol", 272_000, 128_000, false),
            Self::model_info("gpt-5.6-terra", "GPT-5.6 Terra", 272_000, 128_000, false),
            Self::model_info("gpt-5.6-luna", "GPT-5.6 Luna", 272_000, 128_000, false),
        ];
        if self.using_chatgpt_backend() {
            models.retain(|model| Self::chatgpt_supported_models().contains(&model.id.as_str()));
        } else {
            models.retain(|model| !Self::api_key_hidden_model(&model.id));
        }
        models
    }
}

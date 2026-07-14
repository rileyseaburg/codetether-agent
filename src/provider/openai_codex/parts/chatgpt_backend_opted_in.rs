impl OpenAiCodexProvider {
    fn chatgpt_backend_opted_in() -> bool {
        let value = std::env::var(CHATGPT_BACKEND_OPT_IN_ENV);
        value.is_ok_and(|value| matches!(value.trim(), "1" | "true" | "yes" | "on"))
    }
}

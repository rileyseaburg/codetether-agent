impl OpenAiCodexProvider {
    fn format_openai_api_error(status: StatusCode, body: &str, model: &str) -> String {
        if status == StatusCode::UNAUTHORIZED && body.contains("Missing scopes: model.request") {
            return format!(
                "OpenAI Codex OAuth token is missing required scope `model.request` for model `{model}`.\n\
                 Re-run `codetether auth codex` and complete OAuth approval with a ChatGPT subscription account \
                 that has model access in your org/project."
            );
        }
        if status == StatusCode::UNAUTHORIZED
            && (body.contains("chatgpt-account-id")
                || body.contains("ChatGPT-Account-ID")
                || body.contains("workspace"))
        {
            return "OpenAI Codex auth is missing a ChatGPT workspace/account identifier.\n\
                    Re-run `codetether auth codex --device-code` and sign in to the intended workspace."
                .to_string();
        }
        format!("OpenAI API error ({status}): {body}")
    }
}

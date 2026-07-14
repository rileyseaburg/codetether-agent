impl OpenAiCodexProvider {
    fn extract_chatgpt_account_id_from_jwt(jwt: &str) -> Option<String> {
        let payload_b64 = jwt.split('.').nth(1)?;
        let payload = URL_SAFE_NO_PAD.decode(payload_b64).ok()?;
        let value: Value = serde_json::from_slice(&payload).ok()?;

        value
            .get("https://api.openai.com/auth")
            .and_then(Value::as_object)
            .and_then(|auth| auth.get("chatgpt_account_id"))
            .and_then(Value::as_str)
            .map(|s| s.to_string())
            .or_else(|| {
                value
                    .get("chatgpt_account_id")
                    .and_then(Value::as_str)
                    .map(|s| s.to_string())
            })
            .or_else(|| {
                value
                    .get("organizations")
                    .and_then(Value::as_array)
                    .and_then(|orgs| orgs.first())
                    .and_then(|org| org.get("id"))
                    .and_then(Value::as_str)
                    .map(|s| s.to_string())
            })
    }
}

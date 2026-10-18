//! Choose a configured provider using request precedence.
use crate::{OpenAiApiResult, canonicalize_provider_name, openai_bad_request};
pub(crate) fn choose(maybe_provider: Option<&str>, providers: &[&str]) -> OpenAiApiResult<String> {
    Ok(if let Some(provider_name) = maybe_provider {
        let provider_name = canonicalize_provider_name(provider_name);
        if providers.contains(&provider_name) {
            provider_name.to_string()
        } else {
            return Err(openai_bad_request(format!(
                "Provider `{provider_name}` is not configured in Vault"
            )));
        }
    } else if providers.len() == 1 {
        providers[0].to_string()
    } else if providers.contains(&"openai") {
        "openai".to_string()
    } else {
        return Err(openai_bad_request(
            "When multiple providers are configured, use `provider/model`. See GET /v1/models.",
        ));
    })
}

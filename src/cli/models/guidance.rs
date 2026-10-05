//! Recovery hints belong on stderr so JSON capability output stays parseable.
use super::types::ProviderCapability;

pub(super) fn empty_models(capabilities: &[ProviderCapability]) -> Option<String> {
    capabilities.iter().all(|provider| provider.models.is_empty()).then(|| format!(
        "No models were discovered; a configured provider is not proof of authenticated model access.\n\
         If you use Vault, follow this workflow:\n{}\n\
         If you intentionally use local AWS credentials, check that profile and region; see codetether auth bedrock --help.\n\
         If you used --provider, also check the provider name.",
        crate::cli::vault::help::WORKFLOW,
    ))
}

#[cfg(test)]
#[path = "guidance_tests.rs"]
mod tests;

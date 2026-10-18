//! Codex and OpenRouter effort update sequencing.
use super::{Cycle, Runtime};
pub(super) fn advance<R: Runtime>(model: &str, setting: Cycle) -> Option<String> {
    let levels = R::levels(setting, model);
    match setting {
        Cycle::CodexEffort => {
            if levels.is_empty() {
                return None;
            }
            let current = R::current(setting);
            let next = crate::next_codex_effort(current.as_deref(), &levels);
            R::set(setting, next.map(String::from));
            Some(format!(
                "Codex thinking effort: {}",
                crate::reasoning_label(R::current(setting))
            ))
        }
        Cycle::OpenRouterEffort => {
            let next = crate::next_openrouter_effort(R::current(setting).as_deref(), &levels);
            R::set(setting, next.map(String::from));
            Some(format!(
                "OpenRouter thinking effort: {}",
                crate::reasoning_label(R::current(setting))
            ))
        }
        Cycle::BedrockTier | Cycle::BedrockEffort => {
            unreachable!("Bedrock cycles use their own handlers")
        }
    }
}

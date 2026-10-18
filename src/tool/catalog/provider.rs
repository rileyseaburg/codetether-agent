//! Provider-bound constructors preserving clone and ownership transfer order.
use super::*;
pub(super) fn register(host: &mut Native<'_>, kind: ToolKind) -> bool {
    match kind {
        ToolKind::ContextSummarizeProvider => {
            let (provider, model) = host.provider.as_ref().expect("provider catalog mode");
            host.registry
                .register(Arc::new(context_summarize::ContextSummarizeTool::new(
                    Arc::clone(provider),
                    model.clone(),
                )));
        }
        ToolKind::Rlm => {
            let (provider, model) = host.provider.as_ref().expect("provider catalog mode");
            host.registry.register(Arc::new(rlm::RlmTool::new(
                Arc::clone(provider),
                model.clone(),
                crate::rlm::RlmConfig::default(),
            )));
        }
        ToolKind::SessionRecall => {
            let (provider, model) = host.provider.as_ref().expect("provider catalog mode");
            host.registry
                .register(Arc::new(session_recall::SessionRecallTool::new(
                    Arc::clone(provider),
                    model.clone(),
                    crate::rlm::RlmConfig::default(),
                )));
        }
        ToolKind::RalphProvider => {
            let (provider, model) = host.provider.take().expect("provider catalog mode");
            host.registry
                .register(Arc::new(ralph::RalphTool::with_provider(provider, model)));
        }
        _ => return false,
    }
    true
}

//! Ordered application tool catalog with provider and scripting variants.
//!
//! Implement [Host] using native constructors, then call [populate]. The shared
//! catalog owns registration order; the host owns runtime-bound tool instances.
/// Group expansion and provider-first scalar registration policy.
pub mod dispatch;
pub mod groups;
mod kind;
pub use kind::ToolKind;
/// Native construction and registration requested by the catalog.
pub trait Host {
    /// Construct and register the requested tool or registration group.
    fn register(&mut self, kind: ToolKind);
    /// Register a fresh tool only under its compatibility alias.
    fn register_alias(&mut self, alias: &str, kind: ToolKind);
    /// Register a tool and an alias sharing the same constructed instance.
    fn register_with_alias(&mut self, alias: &str, kind: ToolKind);
}
/// Populate tools in their original order for the chosen application mode.
/// # Arguments
/// * `host` — Native constructor and registry bindings.
/// * `provider` — Whether provider-backed tools are available.
/// * `scripting` — Whether the application compiled scripting support.
/// # Returns
/// Registers the catalog, ending with compatibility aliases.
/// # Examples
/// ```no_run
/// # fn setup(host: &mut impl codetether_tool_registry::catalog::Host) {
/// codetether_tool_registry::catalog::populate(host, false, false);
/// # }
/// ```
pub fn populate(host: &mut impl Host, provider: bool, scripting: bool) {
    for kind in [
        ToolKind::Read,
        ToolKind::Write,
        ToolKind::List,
        ToolKind::Glob,
        ToolKind::Tree,
        ToolKind::FileInfo,
        ToolKind::HeadTail,
        ToolKind::Diff,
        ToolKind::Git,
        ToolKind::SearchGroup,
        ToolKind::AdvancedEdit,
        ToolKind::Edit,
        ToolKind::CommandGroup,
        ToolKind::Lsp,
        ToolKind::WebFetch,
        ToolKind::MultiEdit,
        ToolKind::WebSearch,
        ToolKind::BrowserCtl,
        ToolKind::ApplyPatch,
        ToolKind::TodoRead,
        ToolKind::TodoWrite,
        ToolKind::GoalGroup,
        ToolKind::ContextReset,
        ToolKind::ContextBrowse,
        ToolKind::ContextBudget,
        ToolKind::ContextPin,
    ] {
        host.register(kind);
    }
    host.register(if provider {
        ToolKind::ContextSummarizeProvider
    } else {
        ToolKind::ContextSummarizeCached
    });
    for kind in [
        ToolKind::Question,
        ToolKind::Task,
        ToolKind::PlanEnter,
        ToolKind::PlanExit,
        ToolKind::Skill,
        ToolKind::Memory,
    ] {
        host.register(kind);
    }
    if provider {
        host.register(ToolKind::Rlm);
        host.register(ToolKind::SessionRecall);
        host.register(ToolKind::RalphProvider);
    } else {
        host.register(ToolKind::RalphDefault);
    }
    for kind in [
        ToolKind::Prd,
        ToolKind::Undo,
        ToolKind::Voice,
        ToolKind::VoiceInput,
        ToolKind::VoiceStream,
        ToolKind::MediaGroup,
        ToolKind::ConfirmEdit,
        ToolKind::ConfirmMultiEdit,
        ToolKind::SwarmShare,
        ToolKind::Invalid,
        ToolKind::CollaborationGroup,
        ToolKind::SwarmExecute,
        ToolKind::RelayAutoChat,
        ToolKind::Go,
        ToolKind::K8s,
    ] {
        host.register(kind);
    }
    if scripting {
        host.register(ToolKind::ScriptingGroup);
    }
    host.register(ToolKind::CompatibilityGroup);
}

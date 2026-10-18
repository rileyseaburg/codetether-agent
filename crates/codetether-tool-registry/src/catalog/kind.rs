//! Tool and registration-group identities in the application catalog.
/// One native tool constructor or cohesive registration group.
#[derive(Debug, Clone, Copy)]
pub enum ToolKind {
    /// Register read.
    Read,
    /// Register write.
    Write,
    /// Register list.
    List,
    /// Register glob.
    Glob,
    /// Register tree.
    Tree,
    /// Register file info.
    FileInfo,
    /// Register head tail.
    HeadTail,
    /// Register diff.
    Diff,
    /// Register git.
    Git,
    /// Register search group.
    SearchGroup,
    /// Register advanced edit.
    AdvancedEdit,
    /// Register edit.
    Edit,
    /// Register command group.
    CommandGroup,
    /// Register lsp.
    Lsp,
    /// Register web fetch.
    WebFetch,
    /// Register multi edit.
    MultiEdit,
    /// Register web search.
    WebSearch,
    /// Register browser ctl.
    BrowserCtl,
    /// Register apply patch.
    ApplyPatch,
    /// Register todo read.
    TodoRead,
    /// Register todo write.
    TodoWrite,
    /// Register goal group.
    GoalGroup,
    /// Register context reset.
    ContextReset,
    /// Register context browse.
    ContextBrowse,
    /// Register context budget.
    ContextBudget,
    /// Register context pin.
    ContextPin,
    /// Register context summarize cached.
    ContextSummarizeCached,
    /// Register question.
    Question,
    /// Register task.
    Task,
    /// Register plan enter.
    PlanEnter,
    /// Register plan exit.
    PlanExit,
    /// Register skill.
    Skill,
    /// Register memory.
    Memory,
    /// Register ralph default.
    RalphDefault,
    /// Register prd.
    Prd,
    /// Register undo.
    Undo,
    /// Register voice.
    Voice,
    /// Register voice input.
    VoiceInput,
    /// Register voice stream.
    VoiceStream,
    /// Register media group.
    MediaGroup,
    /// Register confirm edit.
    ConfirmEdit,
    /// Register confirm multi edit.
    ConfirmMultiEdit,
    /// Register swarm share.
    SwarmShare,
    /// Register invalid.
    Invalid,
    /// Register collaboration group.
    CollaborationGroup,
    /// Register swarm execute.
    SwarmExecute,
    /// Register relay auto chat.
    RelayAutoChat,
    /// Register go.
    Go,
    /// Register k8s.
    K8s,
    /// Register scripting group.
    ScriptingGroup,
    /// Register compatibility group.
    CompatibilityGroup,
    /// Register context summarize provider.
    ContextSummarizeProvider,
    /// Register rlm.
    Rlm,
    /// Register session recall.
    SessionRecall,
    /// Register ralph provider.
    RalphProvider,
}

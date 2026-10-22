//! Tool and registration-group identities in the application catalog.
/// One native tool constructor or cohesive registration group.
#[derive(Debug, Clone, Copy)]
pub enum ToolKind {
    /// Plan first-party Rust build commands without executing them.
    RustBuildPlan,
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
    /// Register the Grep capability.
    Grep,
    /// Register the Ripgrep capability.
    Ripgrep,
    /// Register the CodeSearch capability.
    CodeSearch,
    /// Register the TokenCount capability.
    TokenCount,
    /// Register the Podcast capability.
    Podcast,
    /// Register the YouTube capability.
    YouTube,
    /// Register the Avatar capability.
    Avatar,
    /// Register the Image capability.
    Image,
    /// Register the ImageGeneration capability.
    ImageGeneration,
    /// Register the McpBridge capability.
    McpBridge,
    /// Register the RustyRoad capability.
    RustyRoad,
    /// Register the Okr capability.
    Okr,
    /// Register the BusInspect capability.
    BusInspect,
    /// Register the ComputerUse capability.
    ComputerUse,
    /// Register the SessionTask tool.
    SessionTask,
    /// Register the GetGoal tool.
    GetGoal,
    /// Register the CreateGoal tool.
    CreateGoal,
    /// Register the UpdateGoal tool.
    UpdateGoal,
    /// Register the EditGoal tool.
    EditGoal,
    /// Register the Agent tool.
    Agent,
    /// Register the SpawnAgent tool.
    SpawnAgent,
    /// Register the FollowupTask tool.
    FollowupTask,
    /// Register the ListAgents tool.
    ListAgents,
    /// Register the WaitAgent tool.
    WaitAgent,
    /// Register the InterruptAgent tool.
    InterruptAgent,
    /// Register the CloseAgent tool.
    CloseAgent,
    /// Register the ResumeAgent tool.
    ResumeAgent,
    /// Register the SendInput tool.
    SendInput,
    /// Register the SendMessage tool.
    SendMessage,
    /// Register the TetherScript tool.
    TetherScript,
}

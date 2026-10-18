//! Native presentation operations selected by navigation policy.
/// One operation applied to the current UI.
#[derive(Debug, Clone)]
pub enum Action {
    /// Apply close search to the current view.
    CloseSearch,
    /// Apply status to the current view.
    Status(String),
    /// Apply set help to the current view.
    SetHelp(bool),
    /// Apply help reset to the current view.
    HelpReset,
    /// Apply close model picker to the current view.
    CloseModelPicker,
    /// Apply clear session filter to the current view.
    ClearSessionFilter,
    /// Apply return to chat to the current view.
    ReturnToChat,
    /// Apply disable file picker to the current view.
    DisableFilePicker,
    /// Apply swarm exit detail to the current view.
    SwarmExitDetail,
    /// Apply ralph exit detail to the current view.
    RalphExitDetail,
    /// Apply bus exit filter to the current view.
    BusExitFilter,
    /// Apply bus exit detail to the current view.
    BusExitDetail,
    /// Apply normal input to the current view.
    NormalInput,
    /// Apply help up to the current view.
    HelpUp(usize),
    /// Apply help down to the current view.
    HelpDown(usize, usize),
    /// Apply symbol previous to the current view.
    SymbolPrevious,
    /// Apply symbol next to the current view.
    SymbolNext,
    /// Apply session previous to the current view.
    SessionPrevious,
    /// Apply session next to the current view.
    SessionNext,
    /// Apply model previous to the current view.
    ModelPrevious,
    /// Apply model next to the current view.
    ModelNext,
    /// Apply settings previous to the current view.
    SettingsPrevious,
    /// Apply settings next to the current view.
    SettingsNext,
    /// Apply slash previous to the current view.
    SlashPrevious,
    /// Apply slash next to the current view.
    SlashNext,
    /// Apply history previous to the current view.
    HistoryPrevious,
    /// Apply history next to the current view.
    HistoryNext,
    /// Apply tool up to the current view.
    ToolUp(usize),
    /// Apply tool down to the current view.
    ToolDown(usize),
    /// Apply chat up to the current view.
    ChatUp(usize),
    /// Apply chat down to the current view.
    ChatDown(usize),
    /// Apply swarm previous to the current view.
    SwarmPrevious,
    /// Apply swarm next to the current view.
    SwarmNext,
    /// Apply ralph detail up to the current view.
    RalphDetailUp(usize),
    /// Apply ralph detail down to the current view.
    RalphDetailDown(usize),
    /// Apply ralph previous to the current view.
    RalphPrevious,
    /// Apply ralph next to the current view.
    RalphNext,
    /// Apply bus detail up to the current view.
    BusDetailUp(usize),
    /// Apply bus detail down to the current view.
    BusDetailDown(usize),
    /// Apply bus previous to the current view.
    BusPrevious,
    /// Apply bus next to the current view.
    BusNext,
    /// Apply file previous to the current view.
    FilePrevious,
    /// Apply file next to the current view.
    FileNext,
    /// Apply swarm detail up to the current view.
    SwarmDetailUp(usize),
    /// Apply swarm detail down to the current view.
    SwarmDetailDown(usize),
    /// Apply file page up to the current view.
    FilePageUp,
    /// Apply file page down to the current view.
    FilePageDown,
    /// Apply chat page up to the current view.
    ChatPageUp,
    /// Apply chat page down to the current view.
    ChatPageDown,
    /// Apply protocol scroll to the current view.
    ProtocolScroll(usize),
    /// Apply cursor word left to the current view.
    CursorWordLeft,
    /// Apply cursor word right to the current view.
    CursorWordRight,
    /// Apply cursor left to the current view.
    CursorLeft,
    /// Apply cursor right to the current view.
    CursorRight,
    /// Apply delete forward to the current view.
    DeleteForward,
    /// Apply cursor home to the current view.
    CursorHome,
    /// Apply cursor end to the current view.
    CursorEnd,
    /// Apply file first to the current view.
    FileFirst,
    /// Apply file last to the current view.
    FileLast,
    /// Apply subagent detail to the current view.
    SubagentDetail(bool),
    /// Apply subagent scroll to the current view.
    SubagentScroll(usize),
    /// Apply child previous to the current view.
    ChildPrevious,
    /// Apply child next to the current view.
    ChildNext,
}

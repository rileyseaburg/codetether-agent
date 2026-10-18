use codetether_tui_models::ViewMode;
/// Native UI operation selected by a modifier shortcut.
/// # Examples
/// ```
/// use codetether_tui_navigation::modified::Action;
/// assert!(matches!(Action::CopyReply, Action::CopyReply));
/// ```
pub enum Action {
    /// Open symbol search.
    SymbolSearch,
    /// Compose a side question.
    SideQuestion,
    /// Toggle chat layout.
    ToggleLayout,
    /// Clear visible transcript messages.
    ClearMessages,
    /// Clear streaming text.
    ClearStreaming,
    /// Save the current view's scroll position.
    SaveScroll,
    /// Assign the requested view mode.
    View(ViewMode),
    /// Copy the latest reply.
    CopyReply,
    /// Copy the transcript.
    CopyTranscript,
    /// Trigger voice input.
    Voice,
    /// Paste clipboard contents.
    Paste,
    /// Cancel the watchdog-reported request.
    CancelWatchdog,
}

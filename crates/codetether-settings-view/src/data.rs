//! Current settings values supplied by the application.
/// Values needed to render settings without owning application configuration.
///
/// # Examples
/// ```
/// let view = codetether_settings_view::SettingsView {
///     selected_settings_index: 0, auto_apply_edits: false, allow_network: false,
///     slash_autocomplete: true, use_worktree: false, access_mode: "ask".into(),
///     provider_labels: ["default".into(), "medium".into(), "default".into(), "default".into()],
/// };
/// assert_eq!(view.selected_settings_index, 0);
/// ```
pub struct SettingsView {
    /// Zero-based selected setting, including provider options.
    pub selected_settings_index: usize,
    /// Whether edit previews apply automatically.
    pub auto_apply_edits: bool,
    /// Whether sandboxed commands may access the network.
    pub allow_network: bool,
    /// Whether slash autocomplete is enabled.
    pub slash_autocomplete: bool,
    /// Whether worktree isolation is enabled.
    pub use_worktree: bool,
    /// Current tool access mode label.
    pub access_mode: String,
    /// Bedrock tier, Bedrock effort, Codex effort and OpenRouter effort labels.
    pub provider_labels: [String; 4],
}

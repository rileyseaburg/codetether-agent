//! Boolean preference identity and existing command/status labels.
/// Boolean preferences exposed as interactive slash commands.
/// # Examples
/// ```
/// use codetether_settings_policy::command::Setting;
/// let setting = Setting::Network;
/// assert_eq!(setting.command(), "network");
/// ```
#[derive(Clone, Copy)]
pub enum Setting {
    /// Automatically apply accepted edits.
    AutoApply,
    /// Allow network access in command execution.
    Network,
    /// Enable slash-command autocomplete.
    Autocomplete,
}
impl Setting {
    /// Return the slash-command name without its leading slash.
    pub fn command(self) -> &'static str {
        match self {
            Self::AutoApply => "autoapply",
            Self::Network => "network",
            Self::Autocomplete => "autocomplete",
        }
    }
    /// Format the existing status message for the current preference.
    pub fn status(self, enabled: bool) -> String {
        match self {
            Self::AutoApply => crate::auto_apply_status_message(enabled),
            Self::Network => crate::network_access_status_message(enabled),
            Self::Autocomplete => crate::autocomplete_status_message(enabled),
        }
    }
}

//! Physical input and explicitly requested background window-message input.

/// Select how mouse/keyboard events are delivered on Windows.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tool::computer_use::input::InputMode;
/// assert_eq!(InputMode::default(), InputMode::Physical);
/// match InputMode::Shadow {
///     InputMode::Physical => unreachable!(),
///     InputMode::Shadow => {},
/// }
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InputMode {
    /// Existing OS cursor/keyboard injection behavior.
    #[default]
    Physical,
    /// Targeted window messages, without a physical-input fallback.
    Shadow,
}

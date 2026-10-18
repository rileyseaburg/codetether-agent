//! Borrowed display fields for a CLI objective confirmation.
/// Objective display information prepared by the CLI.
/// # Examples
/// ```
/// let okr = codetether_okr::Okr::new("Objective", "Description");
/// let prompt = codetether_relay_okr::cli_approval::Prompt { easy_go_requested: true, task_preview: "task", agent_count: 2, model: "model", draft_note: None, okr: &okr };
/// assert_eq!(prompt.agent_count, 2);
/// ```
pub struct Prompt<'a> {
    /// Select the go versus autochat heading.
    pub easy_go_requested: bool,
    /// Task text already truncated for display.
    pub task_preview: &'a str,
    /// Requested specialist count.
    pub agent_count: usize,
    /// Selected model.
    pub model: &'a str,
    /// Optional draft fallback explanation.
    pub draft_note: Option<&'a str>,
    /// Objective and key results to display.
    pub okr: &'a codetether_okr::Okr,
}

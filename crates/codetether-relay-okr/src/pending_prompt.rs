//! Full approval prompt describing a relay objective and key results.
use crate::PendingOkrApproval;
impl PendingOkrApproval {
    /// Get the approval prompt text
    /// # Examples
    /// ```
    /// let pending = codetether_relay_okr::PendingOkrApproval::new("task".into(), 2, "model".into());
    /// assert!(pending.approval_prompt().contains("Press [A] to approve"));
    /// ```
    pub fn approval_prompt(&self) -> String {
        let krs: Vec<String> = self
            .okr
            .key_results
            .iter()
            .map(|kr| format!("  • {} (target: {} {})", kr.title, kr.target_value, kr.unit))
            .collect();

        let note_line = self
            .draft_note
            .as_deref()
            .map(|note| format!("{note}\n"))
            .unwrap_or_default();

        format!(
            "⚠️  Relay OKR Draft\n\n\
             Task: {task}\n\
             Agents: {agents} | Model: {model}\n\n\
             {note_line}\
             Objective: {objective}\n\n\
             Key Results:\n{key_results}\n\n\
             Press [A] to approve or [D] to deny",
            task = self.task.to_string(),
            agents = self.agent_count,
            model = self.model,
            note_line = note_line,
            objective = self.okr.title,
            key_results = krs.join("\n"),
        )
    }
}

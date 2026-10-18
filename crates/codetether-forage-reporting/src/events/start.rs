//! Forage start reporting.
use super::*;
impl RunEvents<'_> {
    /// Announce readiness and audit the initial run options.
    pub async fn start(&self) {
        let _ = self.bus_handle.announce_ready(vec![
            "okr-governance".to_string(),
            "business-prioritization".to_string(),
            "autonomous-forage".to_string(),
        ]);

        log_audit(
            AuditCategory::Cognition,
            "forage.start",
            AuditOutcome::Success,
            Some(json!({
                "top": self.args.top,
                "loop_mode": self.args.loop_mode,
                "interval_secs": self.args.interval_secs,
                "max_cycles": self.args.max_cycles,
                "execute": self.args.execute,
                "execution_engine": self.args.execution_engine,
                "run_timeout_secs": self.args.run_timeout_secs,
                "fail_fast": self.args.fail_fast,
                "swarm_strategy": self.args.swarm_strategy,
                "swarm_max_subagents": self.args.swarm_max_subagents,
                "swarm_max_steps": self.args.swarm_max_steps,
                "swarm_subagent_timeout_secs": self.args.swarm_subagent_timeout_secs,
                "model": self.args.model,
                "moonshot_goals": self.moonshot_rubric.goals.clone(),
                "moonshot_required": self.moonshot_rubric.required,
                "moonshot_min_alignment": self.moonshot_rubric.min_alignment,
            })),
            None,
            None,
        )
        .await;
    }
}

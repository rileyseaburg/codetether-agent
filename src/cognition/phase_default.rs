//! Deterministic phase text used when the thinker is unavailable.

use super::{ThoughtEvent, ThoughtPhase, ThoughtWorkItem, context_summary, text_util};

/// Build deterministic, phase-shaped text for `work`.
pub(super) fn phase_default_text(work: &ThoughtWorkItem, context: &[ThoughtEvent]) -> String {
    let charter = text_util::trim_for_storage(&work.charter, 180);
    let summary = context_summary::summarize(context);
    let thought = match work.phase {
        ThoughtPhase::Observe => format!(
            "Phase: Observe | Goal: detect current customer/business risk | Signals: role={}; charter_focus={}; {} | Uncertainty: live customer-impact telemetry and current incident status are incomplete. | Next_Action: run targeted health/error checks for customer-facing flows and capture failure rate baselines.",
            work.role, charter, summary
        ),
        ThoughtPhase::Reflect => REFLECT.to_string(),
        ThoughtPhase::Test => TEST.to_string(),
        ThoughtPhase::Compress => format!(
            "Phase: Compress | State_Summary: reliability monitoring active with unresolved business-impact uncertainty. | Retained_Facts: role={} ; charter_focus={} ; {} | Open_Risks: potential customer-path instability ; incomplete evidence for confident closure. | Next_Process_Step: convert latest checks into prioritized remediation tasks and verify impact reduction.",
            work.role, charter, summary
        ),
    };
    text_util::trim_for_storage(&thought, 1_200)
}

const REFLECT: &str = "Phase: Reflect | Hypothesis: current instability risk is most likely in runtime reliability and dependency availability. | Rationale: recent context indicates unresolved operational uncertainty. | Business_Risk: outages can cause SLA breach, revenue loss, and trust erosion. | Validation_Next_Action: confirm via service health trend, dependency error distribution, and rollback readiness.";

const TEST: &str = "Phase: Test | Check: verify customer-path service health against recent error spikes and release changes. | Procedure: collect latest health status, error counts, and recent deploy diffs; compare against baseline. | Expected_Result: pass if health is stable and error rate within baseline, fail otherwise. | Evidence_Quality: medium (depends on telemetry completeness). | Escalation_Trigger: escalate immediately on repeated customer-path failures or sustained elevated error rate.";

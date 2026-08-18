//! Per-phase output format instructions for thought prompts.

use super::ThoughtPhase;

/// Return the exact line-label format required for `phase`.
pub(super) fn instruction(phase: ThoughtPhase) -> &'static str {
    match phase {
        ThoughtPhase::Observe => {
            "Process format (exact line labels): \
Phase: Observe | Goal: detect current customer/business risk | \
Signals: 1-3 concrete signals separated by '; ' | \
Uncertainty: one unknown that blocks confidence | \
Next_Action: one immediate operational action."
        }
        ThoughtPhase::Reflect => {
            "Process format (exact line labels): \
Phase: Reflect | Hypothesis: single testable hypothesis | \
Rationale: why this is likely | \
Business_Risk: customer/revenue/SLA impact | \
Validation_Next_Action: one action to confirm or falsify."
        }
        ThoughtPhase::Test => {
            "Process format (exact line labels): \
Phase: Test | Check: single concrete check | \
Procedure: short executable procedure | \
Expected_Result: pass/fail expectation | \
Evidence_Quality: low|medium|high with reason | \
Escalation_Trigger: when to escalate immediately."
        }
        ThoughtPhase::Compress => {
            "Process format (exact line labels): \
Phase: Compress | State_Summary: current state in one line | \
Retained_Facts: 3 short facts separated by '; ' | \
Open_Risks: up to 2 unresolved risks separated by '; ' | \
Next_Process_Step: next operational step."
        }
    }
}

//! Render runtime capability guidance.
use super::capability::RuntimeCapability;

const ALL: &[RuntimeCapability] = &[
    RuntimeCapability::ParallelScheduler,
    RuntimeCapability::SpeculativePrefetch,
    RuntimeCapability::EvidenceClassifier,
    RuntimeCapability::BackgroundContextIndex,
    RuntimeCapability::ToolOutputDigest,
    RuntimeCapability::DynamicTetherScript,
    RuntimeCapability::EventBusWorkflow,
    RuntimeCapability::TypedWorkflowGate,
    RuntimeCapability::EndToEndScopeGate,
];

/// Render runtime capability guidance.
/// # Arguments
/// This renderer takes no arguments.
/// # Returns
/// The established instruction text, with its original labels and ordering.
/// # Examples
/// ```
/// let text = codetether_evidence_prompt::capabilities::render();
/// assert!(!text.is_empty());
/// ```
pub fn render() -> String {
    let mut out = String::from("Runtime acceleration rules:");
    for capability in ALL {
        out.push_str("\n- ");
        out.push_str(capability.instruction());
    }
    out
}

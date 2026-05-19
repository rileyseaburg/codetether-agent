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

pub(crate) fn render() -> String {
    let mut out = String::from("Runtime acceleration rules:");
    for capability in ALL {
        out.push_str("\n- ");
        out.push_str(capability.instruction());
    }
    out
}

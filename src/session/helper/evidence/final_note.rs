use super::gate_mode::GateMode;

pub(crate) fn render(mode: GateMode, answer: &str, session_id: &str, missing: usize) -> String {
    let path = super::ledger_path::for_session(session_id);
    let prefix = match mode {
        GateMode::Strict => "Runtime scope gate STRICT",
        GateMode::Warn => "Runtime scope ledger",
        GateMode::Off => return answer.to_string(),
    };
    format!(
        "{answer}\n\n{prefix}: {missing} deliverable(s) were not explicitly classified as proven, pending, blocked, or not-run. Ledger: {}",
        path.display()
    )
}

//! High-volume, varied tool output surrounding short task-critical messages.
pub(super) fn source() -> String {
    let mut source = String::from(
        "[0 User]\nUse the current local server, not the undeployed hostname.\n\n---\n\n",
    );
    for index in 1..15 {
        source.push_str(&format!("[{index} Tool]\n[ToolResult id=call-{index}]\n"));
        for line in 0..2500 {
            source.push_str(&format!("Document {index} line {line}: primary goal current state last actions pending tasks critical details next steps.\n"));
        }
        source.push_str(&format!(
            "Observed result {index}: saved local record.\n\n---\n\n"
        ));
    }
    source.push_str("[15 User]\nTest through the funnel using the example I supplied.\n\n---\n\n");
    source
}

use codetether_agent::tool::{Tool, bash::BashTool};
use serde_json::json;

#[cfg(unix)]
#[tokio::test]
async fn bash_tool_denies_controlling_tty() {
    let tool = BashTool::with_timeout(5);
    let result = tool
        .execute(json!({
            "command": r#"err="$(mktemp)"; if printf codetether-test >/dev/tty 2>"$err"; then echo tty-opened; else echo no-controlling-tty; cat "$err"; fi; rm -f "$err";"#
        }))
        .await
        .unwrap();

    assert!(
        !result.output.contains("tty-opened"),
        "command unexpectedly opened /dev/tty:\n{}",
        result.output
    );
    assert!(
        result.output.contains("no-controlling-tty"),
        "command did not report tty isolation:\n{}",
        result.output
    );
}

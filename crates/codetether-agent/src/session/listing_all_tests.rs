use super::*;
use std::path::Path;

#[tokio::test]
async fn test_list_sessions_real_directory() {
    let dir = Path::new("/home/riley/A2A-Server-MCP/codetether-agent");
    let result = list_all_sessions_for_directory(dir).await;
    match &result {
        Ok(sessions) => eprintln!("Found {} sessions", sessions.len()),
        Err(err) => eprintln!("Error: {err}"),
    }
    assert!(
        result.is_ok(),
        "list_all_sessions_for_directory should not error"
    );
}

#[test]
fn test_list_codex_sessions_real_directory() {
    let dir = Path::new("/home/riley/A2A-Server-MCP/codetether-agent");
    let result = list_codex_sessions_for_directory(dir);
    match &result {
        Ok(sessions) => eprintln!("Found {} codex sessions", sessions.len()),
        Err(err) => eprintln!("Codex error: {err}"),
    }
    assert!(
        result.is_ok(),
        "list_codex_sessions_for_directory should not error"
    );
}

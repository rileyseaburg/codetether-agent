use codetether_agent::tool::{Tool, voice::VoiceTool, voice_input::encoder::encode_wav};
use serde_json::json;
use tempfile::tempdir;

use crate::{
    env::{EnvGuard, voice_env_lock},
    stub::spawn_voice_stub,
};

#[tokio::test]
async fn voice_tool_actions_hit_live_stub_end_to_end() -> anyhow::Result<()> {
    let _lock = voice_env_lock().lock().await;
    let (base_url, requests, handle) = spawn_voice_stub().await?;
    let _api = EnvGuard::set("CODETETHER_VOICE_API_URL", &base_url);
    let dir = tempdir()?;
    let prev_cwd = std::env::current_dir()?;
    std::env::set_current_dir(dir.path())?;

    let tool = VoiceTool::new();
    assert!(tool.execute(json!({"action":"health"})).await?.success);
    assert!(tool.execute(json!({"action":"list_voices"})).await?.success);

    let wav_path = dir.path().join("sample.wav");
    std::fs::write(&wav_path, encode_wav(&[0, 1024, -1024])?)?;
    let transcribe = tool
        .execute(json!({"action":"transcribe","file_path": wav_path.display().to_string()}))
        .await?;
    assert_eq!(transcribe.output, "stub transcript");

    let speak = tool.execute(json!({"action":"speak","text":"hello from test","voice_id":"960f89fc","language":"english"})).await?;
    assert!(speak.success, "{}", speak.output);
    assert!(dir.path().join("voice_voice-job-1.wav").exists());

    let log = requests.lock().await;
    assert!(
        log.iter()
            .any(|(path, kind, _)| path == "/transcribe" && kind.contains("multipart/form-data"))
    );
    assert!(
        log.iter()
            .any(|(path, kind, _)| path == "/voices/960f89fc/speak"
                && kind.contains("multipart/form-data"))
    );
    std::env::set_current_dir(prev_cwd)?;
    handle.abort();
    Ok(())
}

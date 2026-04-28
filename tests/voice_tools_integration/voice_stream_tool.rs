use codetether_agent::tool::{Tool, voice_stream::VoiceStreamTool};
use serde_json::json;

use crate::{
    env::{EnvGuard, voice_env_lock},
    stub::spawn_voice_stub,
};

#[tokio::test]
async fn voice_stream_actions_hit_live_stub_end_to_end() -> anyhow::Result<()> {
    let _lock = voice_env_lock().lock().await;
    let (base_url, requests, handle) = spawn_voice_stub().await?;
    let _api = EnvGuard::set("CODETETHER_VOICE_API_URL", &base_url);
    let _browser = EnvGuard::set("BROWSER", "/usr/bin/true");

    let tool = VoiceStreamTool::new();
    let speak = tool.execute(json!({"action":"speak_stream","text":"hello stream","voice_id":"960f89fc","language":"english"})).await?;
    assert_eq!(
        speak
            .metadata
            .get("job_id")
            .and_then(|value| value.as_str()),
        Some("stream-job-1")
    );

    let play = tool
        .execute(json!({"action":"play","job_id":"stream-job-1"}))
        .await?;
    assert!(play.success, "{}", play.output);

    let log = requests.lock().await;
    let tts_request = log
        .iter()
        .find(|(path, _, _)| path == "/tts/speak")
        .expect("tts request recorded");
    assert!(tts_request.2.contains("\"script\":\"hello stream\""));
    handle.abort();
    Ok(())
}

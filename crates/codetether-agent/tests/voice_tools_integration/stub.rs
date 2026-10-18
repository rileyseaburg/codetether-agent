use std::sync::Arc;

use axum::{
    Router,
    routing::{get, head, post},
};
use tokio::{net::TcpListener, sync::Mutex};

use crate::{
    routes::{health, output_head, speak_voice, transcribe, tts_speak, voices},
    state::{RequestLog, VoiceStubState},
};

pub async fn spawn_voice_stub() -> anyhow::Result<(String, RequestLog, tokio::task::JoinHandle<()>)>
{
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    let base_url = format!("http://{addr}");
    let requests: RequestLog = Arc::new(Mutex::new(Vec::new()));
    let app = Router::new()
        .route("/health", get(health))
        .route("/voices", get(voices))
        .route("/transcribe", post(transcribe))
        .route("/voices/{voice_id}/speak", post(speak_voice))
        .route("/tts/speak", post(tts_speak))
        .route("/outputs/{job_id}", head(output_head).get(output_head))
        .with_state(VoiceStubState {
            base_url: base_url.clone(),
            requests: requests.clone(),
        });
    let handle = tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    Ok((base_url, requests, handle))
}

//! GPU full-model smoke gate; requires verified model data, never a model service.
use crate::cognition::{CandleDevicePreference, ThinkerBackend, ThinkerConfig};
#[test]
#[ignore = "requires an isolated CUDA GPU and BONSAI_MODEL_DIR"]
fn native_bonsai_full_model_smoke() {
    let root = std::path::PathBuf::from(
        std::env::var_os("BONSAI_MODEL_DIR").expect("model directory required"),
    );
    let config = ThinkerConfig {
        enabled: true,
        backend: ThinkerBackend::Candle,
        model: "ternary-bonsai-2-27b-pq2".into(),
        temperature: 0.0,
        max_tokens: 32,
        candle_model_path: Some(
            root.join("Ternary-Bonsai-2-27B-PQ2_0.gguf")
                .to_string_lossy()
                .into_owned(),
        ),
        candle_tokenizer_path: Some(root.join("tokenizer.json").to_string_lossy().into_owned()),
        candle_arch: Some("qwen35".into()),
        candle_device: CandleDevicePreference::Cuda,
        ..ThinkerConfig::default()
    };
    let started = std::time::Instant::now();
    let mut model = super::native_new::load(&config).expect("native model load");
    let result = model
        .think(
            "You are a helpful assistant.",
            "Reply with exactly BONSAI_NATIVE_OK and nothing else.",
        )
        .expect("native inference");
    println!(
        "native_model={} elapsed_ms={} text={:?} prompt_tokens={:?} completion_tokens={:?}",
        result.model,
        started.elapsed().as_millis(),
        result.text,
        result.prompt_tokens,
        result.completion_tokens
    );
    assert_eq!(result.text.trim(), "BONSAI_NATIVE_OK");
    assert!(result.completion_tokens.is_some_and(|n| n > 0));
}

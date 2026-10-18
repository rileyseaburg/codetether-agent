//! Local CUDA config operations.
use super::*;
impl<R: Runtime> LocalCudaProvider<R> {
    pub(super) fn build_config(&self, request: &CompletionRequest) -> Result<ThinkerConfig> {
        let model_path = self.resolve_model_path().ok_or_else(|| {
            anyhow!(
                "Local CUDA requires model path via LOCAL_CUDA_MODEL_PATH or \
                     CODETETHER_LOCAL_CUDA_MODEL_PATH"
            )
        })?;
        let tokenizer_path = self.resolve_tokenizer_path().or_else(|| {
            // Common local layout: tokenizer.json next to model.gguf
            Path::new(&model_path)
                .parent()
                .map(|p| p.join("tokenizer.json"))
                .filter(|p| p.exists())
                .map(|p| p.to_string_lossy().to_string())
        });
        let tokenizer_path = tokenizer_path.ok_or_else(|| {
            anyhow!(
                "Local CUDA requires tokenizer path via LOCAL_CUDA_TOKENIZER_PATH or \
                 CODETETHER_LOCAL_CUDA_TOKENIZER_PATH"
            )
        })?;

        if !Path::new(&model_path).exists() {
            return Err(anyhow!(
                "Local CUDA model path does not exist: {}",
                model_path
            ));
        }
        if !Path::new(&tokenizer_path).exists() {
            return Err(anyhow!(
                "Local CUDA tokenizer path does not exist: {}",
                tokenizer_path
            ));
        }

        let requested_max_tokens = request.max_tokens.unwrap_or(512).max(1);
        let max_tokens_cap = parse_env_usize(
            &["LOCAL_CUDA_MAX_TOKENS", "CODETETHER_LOCAL_CUDA_MAX_TOKENS"],
            1200,
        )
        .max(1);
        let effective_max_tokens = requested_max_tokens.min(max_tokens_cap);

        let mut config = ThinkerConfig {
            enabled: true,
            backend: ThinkerBackend::Candle,
            model: self.model_name.clone(),
            candle_model_path: Some(model_path),
            candle_tokenizer_path: Some(tokenizer_path),
            candle_arch: self.resolve_architecture(),
            candle_device: self.resolve_device_preference(),
            candle_cuda_ordinal: parse_env_usize(
                &["LOCAL_CUDA_ORDINAL", "CODETETHER_LOCAL_CUDA_ORDINAL"],
                0,
            ),
            candle_repeat_penalty: parse_env_f32(
                &[
                    "LOCAL_CUDA_REPEAT_PENALTY",
                    "CODETETHER_LOCAL_CUDA_REPEAT_PENALTY",
                ],
                1.1,
            ),
            candle_repeat_last_n: parse_env_usize(
                &[
                    "LOCAL_CUDA_REPEAT_LAST_N",
                    "CODETETHER_LOCAL_CUDA_REPEAT_LAST_N",
                ],
                64,
            ),
            candle_seed: parse_env_u64(&["LOCAL_CUDA_SEED", "CODETETHER_LOCAL_CUDA_SEED"], 42),
            temperature: request.temperature.unwrap_or_else(|| {
                parse_env_f32(
                    &[
                        "LOCAL_CUDA_TEMPERATURE",
                        "CODETETHER_LOCAL_CUDA_TEMPERATURE",
                    ],
                    0.2,
                )
            }),
            top_p: request.top_p,
            max_tokens: effective_max_tokens,
            ..ThinkerConfig::default()
        };
        config.timeout_ms = parse_env_u64(
            &["LOCAL_CUDA_TIMEOUT_MS", "CODETETHER_LOCAL_CUDA_TIMEOUT_MS"],
            120_000,
        );

        tracing::info!(
            model = %config.model,
            device = ?config.candle_device,
            cuda_ordinal = config.candle_cuda_ordinal,
            requested_max_tokens = requested_max_tokens,
            max_tokens_cap = max_tokens_cap,
            max_tokens = config.max_tokens,
            "Initialized Local CUDA runtime configuration"
        );

        Ok(config)
    }
}

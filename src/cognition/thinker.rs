use anyhow::{Context, Result, anyhow};
use candle_core::quantized::gguf_file;
use candle_core::{DType, Device, Tensor};
use candle_transformers::generation::LogitsProcessor;

use candle_transformers::models::quantized_gemma3;
use candle_transformers::models::{
    quantized_llama, quantized_qwen2, quantized_qwen3, quantized_qwen3_moe,
};
use candle_transformers::utils::apply_repeat_penalty;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokenizers::Tokenizer;

use crate::provider::bedrock::{AwsCredentials, BedrockProvider};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThinkerBackend {
    OpenAICompat,
    Candle,
    Bedrock,
}

impl ThinkerBackend {
    pub fn from_env(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "candle" => Self::Candle,
            "openai" | "openai_compat" | "openai-compatible" | "http" => Self::OpenAICompat,
            "bedrock" | "aws" | "aws_bedrock" => Self::Bedrock,
            _ => Self::OpenAICompat,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandleDevicePreference {
    Auto,
    Cpu,
    Cuda,
}

impl CandleDevicePreference {
    pub fn from_env(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "cpu" => Self::Cpu,
            "cuda" | "gpu" => Self::Cuda,
            _ => Self::Auto,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThinkerConfig {
    pub enabled: bool,
    pub backend: ThinkerBackend,
    pub endpoint: String,
    pub model: String,
    pub api_key: Option<String>,
    pub temperature: f32,
    pub top_p: Option<f32>,
    pub max_tokens: usize,
    pub timeout_ms: u64,
    pub candle_model_path: Option<String>,
    pub candle_tokenizer_path: Option<String>,
    pub candle_arch: Option<String>,
    pub candle_device: CandleDevicePreference,
    pub candle_cuda_ordinal: usize,
    pub candle_repeat_penalty: f32,
    pub candle_repeat_last_n: usize,
    pub candle_seed: u64,
    pub bedrock_region: String,
}

impl Default for ThinkerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            backend: ThinkerBackend::OpenAICompat,
            endpoint: "http://127.0.0.1:11434/v1/chat/completions".to_string(),
            model: "qwen2.5:3b-instruct".to_string(),
            api_key: None,
            temperature: 0.2,
            top_p: None,
            max_tokens: 256,
            timeout_ms: 30_000,
            candle_model_path: None,
            candle_tokenizer_path: None,
            candle_arch: None,
            candle_device: CandleDevicePreference::Auto,
            candle_cuda_ordinal: 0,
            candle_repeat_penalty: 1.1,
            candle_repeat_last_n: 64,
            candle_seed: 42,
            bedrock_region: "us-west-2".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThinkerOutput {
    pub model: String,
    pub finish_reason: Option<String>,
    pub text: String,
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}

#[derive(Clone)]
pub struct ThinkerClient {
    config: ThinkerConfig,
    backend: ThinkerClientBackend,
}

impl std::fmt::Debug for ThinkerClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ThinkerClient")
            .field("backend", &self.config.backend)
            .field("model", &self.config.model)
            .finish()
    }
}

#[derive(Clone)]
enum ThinkerClientBackend {
    OpenAICompat { http: Client },
    Candle { runtime: Arc<Mutex<CandleThinker>> },
    Bedrock { provider: Arc<BedrockProvider> },
}

impl ThinkerClient {
    pub fn new(config: ThinkerConfig) -> Result<Self> {
        let backend = match config.backend {
            ThinkerBackend::OpenAICompat => {
                let timeout = Duration::from_millis(config.timeout_ms.max(1_000));
                let http = Client::builder()
                    .timeout(timeout)
                    .build()
                    .context("failed to build thinker HTTP client")?;
                ThinkerClientBackend::OpenAICompat { http }
            }
            ThinkerBackend::Candle => {
                let runtime = CandleThinker::new(&config)?;
                ThinkerClientBackend::Candle {
                    runtime: Arc::new(Mutex::new(runtime)),
                }
            }
            ThinkerBackend::Bedrock => {
                let creds = AwsCredentials::from_environment()
                    .ok_or_else(|| anyhow!("Bedrock thinker requires AWS credentials (AWS_ACCESS_KEY_ID/AWS_SECRET_ACCESS_KEY or ~/.aws/credentials)"))?;
                let provider =
                    BedrockProvider::with_credentials(creds, config.bedrock_region.clone())?;
                ThinkerClientBackend::Bedrock {
                    provider: Arc::new(provider),
                }
            }
        };

        Ok(Self { config, backend })
    }

    pub fn config(&self) -> &ThinkerConfig {
        &self.config
    }

    pub async fn think(&self, system_prompt: &str, user_prompt: &str) -> Result<ThinkerOutput> {
        match &self.backend {
            ThinkerClientBackend::OpenAICompat { http } => {
                self.think_openai_compat(http, system_prompt, user_prompt)
                    .await
            }
            ThinkerClientBackend::Bedrock { provider } => {
                self.think_bedrock(provider, system_prompt, user_prompt)
                    .await
            }
            ThinkerClientBackend::Candle { runtime } => {
                let runtime = Arc::clone(runtime);
                let system_prompt = system_prompt.to_string();
                let user_prompt = user_prompt.to_string();
                tokio::task::spawn_blocking(move || {
                    let mut guard = match runtime.try_lock() {
                        Ok(g) => g,
                        Err(std::sync::TryLockError::WouldBlock) => {
                            return Err(anyhow!("candle thinker is busy"));
                        }
                        Err(std::sync::TryLockError::Poisoned(_)) => {
                            return Err(anyhow!("candle thinker mutex poisoned"));
                        }
                    };
                    guard.think(&system_prompt, &user_prompt)
                })
                .await
                .context("candle thinker task join failed")?
            }
        }
    }

    async fn think_bedrock(
        &self,
        provider: &BedrockProvider,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<ThinkerOutput> {
        let started_at = Instant::now();
        let model_id = &self.config.model;

        // Build Bedrock Converse request body
        let body = serde_json::json!({
            "system": [{"text": system_prompt}],
            "messages": [{
                "role": "user",
                "content": [{"text": user_prompt}]
            }],
            "inferenceConfig": {
                "maxTokens": self.config.max_tokens,
                "temperature": self.config.temperature
            }
        });

        let body_bytes = serde_json::to_vec(&body)?;
        let encoded_model_id = model_id.replace(':', "%3A");
        let url = format!(
            "https://bedrock-runtime.{}.amazonaws.com/model/{}/converse",
            self.config.bedrock_region, encoded_model_id
        );

        let response = provider
            .send_converse_request(&url, &body_bytes)
            .await
            .context("Bedrock thinker converse request failed")?;

        let status = response.status();
        let text = response
            .text()
            .await
            .context("Failed to read Bedrock thinker response")?;

        if !status.is_success() {
            return Err(anyhow!(
                "Bedrock thinker error ({}): {}",
                status,
                &text[..text.len().min(500)]
            ));
        }

        let parsed: serde_json::Value =
            serde_json::from_str(&text).context("Failed to parse Bedrock thinker response")?;

        let output_text = parsed["output"]["message"]["content"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|c| c["text"].as_str())
            .unwrap_or_default()
            .to_string();

        let usage = &parsed["usage"];
        let prompt_tokens = usage["inputTokens"].as_u64().map(|v| v as u32);
        let completion_tokens = usage["outputTokens"].as_u64().map(|v| v as u32);

        tracing::debug!(
            model = model_id,
            latency_ms = started_at.elapsed().as_millis(),
            prompt_tokens = ?prompt_tokens,
            completion_tokens = ?completion_tokens,
            "bedrock thinker generated thought"
        );

        Ok(ThinkerOutput {
            model: model_id.clone(),
            finish_reason: parsed["stopReason"].as_str().map(|s| s.to_string()),
            text: output_text,
            prompt_tokens,
            completion_tokens,
            total_tokens: prompt_tokens.zip(completion_tokens).map(|(p, c)| p + c),
        })
    }

    async fn think_openai_compat(
        &self,
        http: &Client,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<ThinkerOutput> {
        let started_at = Instant::now();
        let body = OpenAIChatRequest {
            model: self.config.model.clone(),
            messages: vec![
                OpenAIMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                OpenAIMessage {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            temperature: self.config.temperature,
            top_p: self.config.top_p,
            max_tokens: self.config.max_tokens,
            stream: false,
        };

        // Retry once on transient failures (connection errors, 429, 502-504).
        let max_attempts: u32 = 2;
        let mut last_err: Option<anyhow::Error> = None;

        for attempt in 0..max_attempts {
            if attempt > 0 {
                tokio::time::sleep(Duration::from_millis(500 * attempt as u64)).await;
                tracing::debug!(attempt, "retrying thinker HTTP request");
            }

            let mut request = http.post(&self.config.endpoint).json(&body);
            if let Some(key) = self.config.api_key.as_ref() {
                request = request.bearer_auth(key);
            }

            let response = match request.send().await {
                Ok(resp) => resp,
                Err(e) => {
                    if is_transient_reqwest_error(&e) {
                        tracing::warn!(attempt, error = %e, "thinker HTTP request failed (transient)");
                        last_err =
                            Some(anyhow::Error::from(e).context("transient thinker send error"));
                        continue;
                    }
                    return Err(anyhow::Error::from(e).context("non-transient thinker send error"));
                }
            };

            let status = response.status();
            if is_transient_http_error(status.as_u16()) {
                let body_text = response.text().await.unwrap_or_default();
                tracing::warn!(attempt, status = %status, "thinker received transient HTTP error");
                last_err = Some(anyhow!(
                    "thinker request failed with status {}: {}",
                    status,
                    body_text
                ));
                continue;
            }

            if !status.is_success() {
                let body_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "<empty>".to_string());
                return Err(anyhow!(
                    "thinker request failed with status {}: {}",
                    status,
                    body_text
                ));
            }

            let payload: OpenAIChatResponse = response
                .json()
                .await
                .context("failed to decode thinker response")?;
            let choice = payload
                .choices
                .first()
                .ok_or_else(|| anyhow!("thinker response did not include choices"))?;
            let text = choice.message.extract_text();
            let usage = payload.usage.unwrap_or_default();

            let output = ThinkerOutput {
                model: payload.model.unwrap_or_else(|| self.config.model.clone()),
                finish_reason: choice.finish_reason.clone(),
                text,
                prompt_tokens: usage.prompt_tokens,
                completion_tokens: usage.completion_tokens,
                total_tokens: usage.total_tokens,
            };

            tracing::debug!(
                model = %output.model,
                latency_ms = started_at.elapsed().as_millis(),
                prompt_tokens = ?output.prompt_tokens,
                completion_tokens = ?output.completion_tokens,
                attempt,
                "openai-compat thinker generated thought"
            );

            return Ok(output);
        }

        Err(last_err.unwrap_or_else(|| {
            anyhow!("thinker HTTP request failed after {max_attempts} attempts")
        }))
    }
}

pub(crate) struct CandleThinker {
    model: CandleModel,
    tokenizer: Tokenizer,
    device: Device,
    model_label: String,
    architecture: String,
    context_window: usize,
    temperature: f32,
    top_p: Option<f32>,
    max_tokens: usize,
    repeat_penalty: f32,
    repeat_last_n: usize,
    seed: u64,
    request_index: u64,
    eos_token_ids: HashSet<u32>,
    cached_tokens: Vec<u32>,
}

enum CandleModel {
    Llama(quantized_llama::ModelWeights),
    Qwen2(quantized_qwen2::ModelWeights),
    Qwen3(quantized_qwen3::ModelWeights),
    Qwen3Moe(quantized_qwen3_moe::GGUFQWenMoE),

    Gemma3(quantized_gemma3::ModelWeights),
}

impl CandleModel {
    fn forward(&mut self, x: &Tensor, index_pos: usize) -> Result<Tensor> {
        match self {
            Self::Llama(model) => Ok(model.forward(x, index_pos)?),
            Self::Qwen2(model) => Ok(model.forward(x, index_pos)?),
            Self::Qwen3(model) => Ok(model.forward(x, index_pos)?),
            Self::Qwen3Moe(model) => Ok(model.forward(x, index_pos)?),

            Self::Gemma3(model) => Ok(model.forward(x, index_pos)?),
        }
    }
}

impl CandleThinker {
    pub(crate) fn new(config: &ThinkerConfig) -> Result<Self> {
        let model_path = config.candle_model_path.as_ref().ok_or_else(|| {
            anyhow!("candle backend requires CODETETHER_COGNITION_THINKER_CANDLE_MODEL_PATH")
        })?;
        let tokenizer_path = config.candle_tokenizer_path.as_ref().ok_or_else(|| {
            anyhow!("candle backend requires CODETETHER_COGNITION_THINKER_CANDLE_TOKENIZER_PATH")
        })?;

        let (device, device_label) = select_candle_device(config)?;
        let mut reader = BufReader::new(
            File::open(model_path)
                .with_context(|| format!("failed to open candle model file at {}", model_path))?,
        );
        let content = gguf_file::Content::read(&mut reader)
            .with_context(|| format!("failed to parse gguf model metadata from {}", model_path))?;

        let architecture = config
            .candle_arch
            .clone()
            .or_else(|| {
                content
                    .metadata
                    .get("general.architecture")
                    .and_then(|v| v.to_string().ok())
                    .cloned()
            })
            .unwrap_or_else(|| "llama".to_string())
            .to_ascii_lowercase();

        let context_window = detect_context_window(&content, &architecture).unwrap_or(4096);
        let model_label = format!("candle:{}:{}@{}", architecture, device_label, model_path);

        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| anyhow!("failed to load tokenizer from {}: {}", tokenizer_path, e))?;

        // Extract EOS metadata from content before it is moved into from_gguf.
        let gguf_eos_ids = extract_gguf_eos_ids(&content);

        let model = match architecture.as_str() {
            "llama" => CandleModel::Llama(
                quantized_llama::ModelWeights::from_gguf(content, &mut reader, &device)
                    .with_context(|| format!("failed to load llama gguf from {}", model_path))?,
            ),
            "qwen2" => CandleModel::Qwen2(
                quantized_qwen2::ModelWeights::from_gguf(content, &mut reader, &device)
                    .with_context(|| format!("failed to load qwen2 gguf from {}", model_path))?,
            ),
            "qwen3" => CandleModel::Qwen3(
                quantized_qwen3::ModelWeights::from_gguf(content, &mut reader, &device)
                    .with_context(|| format!("failed to load qwen3 gguf from {}", model_path))?,
            ),
            "qwen3moe" | "qwen3_moe" => CandleModel::Qwen3Moe(
                quantized_qwen3_moe::GGUFQWenMoE::from_gguf(
                    content,
                    &mut reader,
                    &device,
                    DType::F16,
                )
                .with_context(|| format!("failed to load qwen3_moe gguf from {}", model_path))?,
            ),

            "gemma" | "gemma2" | "gemma3" | "gemma-embedding" => CandleModel::Gemma3(
                quantized_gemma3::ModelWeights::from_gguf(content, &mut reader, &device)
                    .with_context(|| format!("failed to load gemma3 gguf from {}", model_path))?,
            ),
            other => {
                #[cfg(not(feature = "functiongemma"))]
                if matches!(other, "gemma" | "gemma2" | "gemma3" | "gemma-embedding") {
                    return Err(anyhow!(
                        "gemma architecture '{}' requires the 'functiongemma' feature; rebuild with --features functiongemma",
                        other
                    ));
                }
                return Err(anyhow!(
                    "unsupported candle architecture '{}' (supported: llama, qwen2, qwen3, qwen3_moe{})",
                    other,
                    if cfg!(feature = "functiongemma") {
                        ", gemma/gemma2/gemma3"
                    } else {
                        ""
                    }
                ));
            }
        };

        let eos_token_ids: HashSet<u32> = collect_eos_token_ids(&tokenizer, &gguf_eos_ids);
        if eos_token_ids.is_empty() {
            tracing::warn!(
                "No EOS tokens found in tokenizer; generation will stop on max token limit"
            );
        }

        Ok(Self {
            model,
            tokenizer,
            device,
            model_label,
            architecture,
            context_window,
            temperature: config.temperature,
            top_p: config.top_p,
            max_tokens: config.max_tokens.max(1),
            repeat_penalty: config.candle_repeat_penalty.max(1.0),
            repeat_last_n: config.candle_repeat_last_n.max(1),
            seed: config.candle_seed,
            request_index: 0,
            eos_token_ids,
            cached_tokens: Vec::new(),
        })
    }

    pub(crate) fn think(
        &mut self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<ThinkerOutput> {
        let started_at = Instant::now();
        let prompt = format_chat_prompt(&self.architecture, system_prompt, user_prompt);
        let encoding = self
            .tokenizer
            .encode(prompt.as_str(), true)
            .map_err(|e| anyhow!("tokenizer encode failed: {}", e))?;
        let mut tokens = encoding.get_ids().to_vec();
        if tokens.is_empty() {
            return Err(anyhow!("tokenizer produced an empty prompt token set"));
        }

        // Truncate user content while preserving the system prompt prefix.
        if self.context_window > 8 && tokens.len() >= self.context_window {
            let system_only = format_chat_prompt(&self.architecture, system_prompt, "");
            let sys_encoding = self
                .tokenizer
                .encode(system_only.as_str(), true)
                .map_err(|e| anyhow!("tokenizer encode failed (system): {}", e))?;
            let sys_len = sys_encoding.get_ids().len();
            let budget = self.context_window.saturating_sub(8);
            if sys_len < budget {
                // Keep system prefix + tail of user content that fits
                let tail_budget = budget.saturating_sub(sys_len);
                let tail_start = tokens.len().saturating_sub(tail_budget);
                let mut truncated = sys_encoding.get_ids().to_vec();
                truncated.extend_from_slice(&tokens[tail_start..]);
                tokens = truncated;
            } else {
                // System alone exceeds budget; keep only the tail
                let keep = budget;
                tokens = tokens[tokens.len().saturating_sub(keep)..].to_vec();
            }
        }
        let prompt_token_count = tokens.len() as u32;

        let request_seed = self.seed.wrapping_add(self.request_index);
        self.request_index = self.request_index.wrapping_add(1);
        let mut logits_processor = LogitsProcessor::new(
            request_seed,
            Some(self.temperature as f64),
            self.top_p.map(|v| v as f64),
        );

        let mut index_pos = 0usize;
        let mut generated: Vec<u32> = Vec::with_capacity(self.max_tokens);
        let mut finish_reason = "length".to_string();

        for _ in 0..self.max_tokens {
            let ctxt: &[u32] = if index_pos == 0 {
                tokens.as_slice()
            } else {
                &tokens[tokens.len() - 1..]
            };

            let input = Tensor::new(ctxt, &self.device)?
                .unsqueeze(0)
                .context("failed to create candle input tensor")?;
            let mut logits = self
                .model
                .forward(&input, index_pos)
                .context("candle model forward failed")?;
            index_pos += ctxt.len();
            logits = logits
                .squeeze(0)
                .context("failed to squeeze logits batch dimension")?;

            let logits = if self.repeat_penalty > 1.0 {
                let start_at = tokens.len().saturating_sub(self.repeat_last_n);
                apply_repeat_penalty(&logits, self.repeat_penalty, &tokens[start_at..])
                    .context("failed to apply repeat penalty")?
            } else {
                logits
            };

            let next_token = sample_next_token_with_fallback(&mut logits_processor, &logits)?;
            if self.eos_token_ids.contains(&next_token) {
                finish_reason = "stop".to_string();
                break;
            }

            tokens.push(next_token);
            generated.push(next_token);

            if tokens.len() + 1 >= self.context_window {
                finish_reason = "length".to_string();
                break;
            }
        }

        let text = self
            .tokenizer
            .decode(&generated, true)
            .map_err(|e| anyhow!("tokenizer decode failed: {}", e))?;
        let completion_tokens = generated.len() as u32;

        tracing::debug!(
            model = %self.model_label,
            latency_ms = started_at.elapsed().as_millis(),
            prompt_tokens = prompt_token_count,
            completion_tokens = completion_tokens,
            "candle thinker generated thought"
        );

        Ok(ThinkerOutput {
            model: self.model_label.clone(),
            finish_reason: Some(finish_reason),
            text,
            prompt_tokens: Some(prompt_token_count),
            completion_tokens: Some(completion_tokens),
            total_tokens: Some(prompt_token_count + completion_tokens),
        })
    }
}

/// Build a chat prompt using the proper template for each model architecture.
fn format_chat_prompt(architecture: &str, system_prompt: &str, user_prompt: &str) -> String {
    match architecture {
        // ChatML template (Qwen2, Yi, etc.)
        "qwen2" | "qwen3" | "qwen3moe" | "qwen3_moe" => format!(
            "<|im_start|>system\n{system}<|im_end|>\n<|im_start|>user\n{user}<|im_end|>\n<|im_start|>assistant\n",
            system = system_prompt,
            user = user_prompt,
        ),
        // Llama 3 instruct template
        "llama" => format!(
            "<|begin_of_text|><|start_header_id|>system<|end_header_id|>\n\n{system}<|eot_id|><|start_header_id|>user<|end_header_id|>\n\n{user}<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n",
            system = system_prompt,
            user = user_prompt,
        ),
        // Gemma instruct template
        "gemma" | "gemma2" | "gemma3" | "gemma-embedding" => format!(
            "<start_of_turn>user\n{system}\n\n{user}<end_of_turn>\n<start_of_turn>model\n",
            system = system_prompt,
            user = user_prompt,
        ),
        // Fallback for unknown architectures
        _ => format!(
            "System:\n{system}\n\nUser:\n{user}\n\nAssistant:\n",
            system = system_prompt,
            user = user_prompt,
        ),
    }
}

fn select_candle_device(config: &ThinkerConfig) -> Result<(Device, String)> {
    match config.candle_device {
        CandleDevicePreference::Cpu => Ok((Device::Cpu, "cpu".to_string())),
        CandleDevicePreference::Cuda => {
            let device = try_cuda_device(config.candle_cuda_ordinal)?;
            Ok((device, format!("cuda:{}", config.candle_cuda_ordinal)))
        }
        CandleDevicePreference::Auto => match try_cuda_device(config.candle_cuda_ordinal) {
            Ok(device) => {
                tracing::info!(
                    ordinal = config.candle_cuda_ordinal,
                    "Candle thinker selected CUDA device"
                );
                Ok((device, format!("cuda:{}", config.candle_cuda_ordinal)))
            }
            Err(error) => {
                tracing::warn!(
                    %error,
                    "CUDA unavailable for Candle thinker, falling back to CPU"
                );
                Ok((Device::Cpu, "cpu".to_string()))
            }
        },
    }
}

#[cfg(feature = "candle-cuda")]
fn try_cuda_device(ordinal: usize) -> Result<Device> {
    Device::new_cuda(ordinal)
        .with_context(|| format!("failed to initialize CUDA device ordinal {}", ordinal))
}

#[cfg(not(feature = "candle-cuda"))]
fn try_cuda_device(_ordinal: usize) -> Result<Device> {
    Err(anyhow!(
        "candle-cuda feature is not enabled in this build; rebuild with --features candle-cuda"
    ))
}

fn detect_context_window(content: &gguf_file::Content, architecture: &str) -> Option<usize> {
    let key = match architecture {
        "qwen2" => "qwen2.context_length",
        "qwen3" | "qwen3moe" | "qwen3_moe" => "qwen3.context_length",
        "gemma" | "gemma2" | "gemma3" | "gemma-embedding" => {
            // Try gemma3 first, then fall back to gemma2, gemma
            for prefix in ["gemma3", "gemma2", "gemma"] {
                let k = format!("{prefix}.context_length");
                if let Some(v) = content.metadata.get(&k) {
                    return v.to_u32().ok().map(|v| v as usize);
                }
            }
            return None;
        }
        _ => "llama.context_length",
    };
    content
        .metadata
        .get(key)
        .and_then(|v| v.to_u32().ok())
        .map(|v| v as usize)
}

/// Extract EOS token IDs from GGUF metadata before the content is consumed.
fn extract_gguf_eos_ids(content: &gguf_file::Content) -> Vec<u32> {
    let mut ids = Vec::new();
    for key in ["tokenizer.ggml.eos_token_id", "tokenizer.ggml.eot_token_id"] {
        if let Some(v) = content.metadata.get(key) {
            if let Ok(id) = v.to_u32() {
                if !ids.contains(&id) {
                    ids.push(id);
                }
            }
        }
    }
    ids
}

fn collect_eos_token_ids(tokenizer: &Tokenizer, gguf_eos_ids: &[u32]) -> HashSet<u32> {
    let mut ids: HashSet<u32> = gguf_eos_ids.iter().copied().collect();

    // Also check well-known special token strings as fallback.
    let candidates = [
        "<|im_end|>",
        "<|eot_id|>",
        "<|endoftext|>",
        "</s>",
        "<|end|>",
        "<end_of_turn>",
    ];
    for token in candidates {
        if let Some(id) = tokenizer.token_to_id(token) {
            ids.insert(id);
        }
    }
    ids
}

fn sample_next_token_with_fallback(
    logits_processor: &mut LogitsProcessor,
    logits: &Tensor,
) -> Result<u32> {
    match logits_processor.sample(logits) {
        Ok(token) => Ok(token),
        Err(sample_error) => {
            let logits_vec = logits
                .to_vec1::<f32>()
                .context("token sampling failed and fallback logits extraction failed")?;
            let mut best_token = None;
            let mut best_logit = f32::NEG_INFINITY;

            for (idx, logit) in logits_vec.into_iter().enumerate() {
                if !logit.is_finite() {
                    continue;
                }
                if best_token.is_none() || logit > best_logit {
                    best_token = Some(idx as u32);
                    best_logit = logit;
                }
            }

            if let Some(token) = best_token {
                tracing::warn!(
                    error = %sample_error,
                    token,
                    "Token sampling produced invalid weights; using greedy argmax fallback"
                );
                Ok(token)
            } else {
                Err(anyhow!(
                    "token sampling failed and fallback argmax found no finite logits: {}",
                    sample_error
                ))
            }
        }
    }
}

/// Returns true for HTTP status codes that are worth retrying.
fn is_transient_http_error(status: u16) -> bool {
    matches!(status, 429 | 502 | 503 | 504)
}

/// Returns true for reqwest errors that are worth retrying (timeouts, connection resets).
fn is_transient_reqwest_error(e: &reqwest::Error) -> bool {
    e.is_timeout() || e.is_connect() || e.is_request()
}

#[derive(Debug, Serialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    max_tokens: usize,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIChatResponse {
    model: Option<String>,
    choices: Vec<OpenAIChatChoice>,
    #[serde(default)]
    usage: Option<OpenAIUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChatChoice {
    message: OpenAIChatChoiceMessage,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChatChoiceMessage {
    #[serde(default)]
    content: Option<OpenAIChatContent>,
    #[serde(default)]
    reasoning: Option<String>,
    #[serde(default)]
    reasoning_content: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: Option<u32>,
    completion_tokens: Option<u32>,
    total_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum OpenAIChatContent {
    Text(String),
    Parts(Vec<OpenAIChatContentPart>),
    Part(OpenAIChatContentPart),
}

#[derive(Debug, Deserialize)]
struct OpenAIChatContentPart {
    #[serde(rename = "type")]
    kind: Option<String>,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    content: Option<String>,
}

impl OpenAIChatChoiceMessage {
    fn extract_text(&self) -> String {
        let content_text = self
            .content
            .as_ref()
            .map(OpenAIChatContent::to_text)
            .unwrap_or_default();
        if !content_text.trim().is_empty() {
            return content_text;
        }

        if let Some(reasoning) = self
            .reasoning
            .as_deref()
            .filter(|text| !text.trim().is_empty())
        {
            return reasoning.to_string();
        }

        self.reasoning_content
            .as_deref()
            .filter(|text| !text.trim().is_empty())
            .unwrap_or_default()
            .to_string()
    }
}

impl OpenAIChatContent {
    fn to_text(&self) -> String {
        match self {
            Self::Text(text) => text.clone(),
            Self::Parts(parts) => parts
                .iter()
                .filter_map(OpenAIChatContentPart::text_fragment)
                .collect::<Vec<_>>()
                .join("\n"),
            Self::Part(part) => part.text_fragment().unwrap_or_default(),
        }
    }
}

impl OpenAIChatContentPart {
    fn text_fragment(&self) -> Option<String> {
        if let Some(kind) = self.kind.as_deref()
            && !kind.eq_ignore_ascii_case("text")
            && !kind.eq_ignore_ascii_case("output_text")
        {
            return None;
        }

        self.text
            .as_deref()
            .or(self.content.as_deref())
            .map(str::trim)
            .filter(|text| !text.is_empty())
            .map(ToString::to_string)
    }
}

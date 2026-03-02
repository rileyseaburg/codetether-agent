//! Codebase indexer
//!
//! Builds a lightweight, persistent JSON index of source files for fast
//! workspace introspection and downstream search/ranking workflows.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use walkdir::{DirEntry, WalkDir};

const INDEX_VERSION: u32 = 2;
const LOCAL_EMBEDDING_PROVIDER: &str = "local/hash-embedding";
const DEFAULT_EMBEDDING_PROVIDER: &str = "local";
const DEFAULT_EMBEDDING_MODEL: &str = "hash-v1";
const DEFAULT_EMBEDDING_DIMENSIONS: usize = 384;
const DEFAULT_EMBEDDING_BATCH_SIZE: usize = 32;
const DEFAULT_EMBEDDING_INPUT_CHARS: usize = 8_000;
const DEFAULT_EMBEDDING_MAX_RETRIES: u32 = 3;
const DEFAULT_EMBEDDING_RETRY_INITIAL_MS: u64 = 250;
const DEFAULT_EMBEDDING_RETRY_MAX_MS: u64 = 2_000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodebaseIndex {
    pub version: u32,
    pub root: String,
    pub generated_at: DateTime<Utc>,
    pub embedding_provider: String,
    pub embedding_model: String,
    pub stats: IndexStats,
    pub files: Vec<IndexedFile>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndexStats {
    pub total_seen_files: u64,
    pub indexed_files: u64,
    pub skipped_hidden: u64,
    pub skipped_non_text: u64,
    pub skipped_large: u64,
    pub skipped_io_errors: u64,
    pub total_bytes: u64,
    pub total_lines: u64,
    pub embedded_files: u64,
    pub embedding_dimensions: u32,
    pub embedding_prompt_tokens: u64,
    pub embedding_total_tokens: u64,
    pub language_counts: HashMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedFile {
    pub path: String,
    pub language: String,
    pub bytes: u64,
    pub lines: u32,
    pub symbol_hints: u32,
    pub modified_unix_ms: Option<i64>,
    pub embedding: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct BuildOptions {
    pub include_hidden: bool,
    pub max_file_size_bytes: u64,
    pub embedding_provider: String,
    pub embedding_model: String,
    pub embedding_dimensions: usize,
    pub embedding_batch_size: usize,
    pub embedding_input_chars: usize,
    pub embedding_max_retries: u32,
    pub embedding_retry_initial_ms: u64,
    pub embedding_retry_max_ms: u64,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            include_hidden: false,
            max_file_size_bytes: 1024 * 1024,
            embedding_provider: DEFAULT_EMBEDDING_PROVIDER.to_string(),
            embedding_model: DEFAULT_EMBEDDING_MODEL.to_string(),
            embedding_dimensions: DEFAULT_EMBEDDING_DIMENSIONS,
            embedding_batch_size: DEFAULT_EMBEDDING_BATCH_SIZE,
            embedding_input_chars: DEFAULT_EMBEDDING_INPUT_CHARS,
            embedding_max_retries: DEFAULT_EMBEDDING_MAX_RETRIES,
            embedding_retry_initial_ms: DEFAULT_EMBEDDING_RETRY_INITIAL_MS,
            embedding_retry_max_ms: DEFAULT_EMBEDDING_RETRY_MAX_MS,
        }
    }
}

pub async fn run(args: crate::cli::IndexArgs) -> Result<()> {
    let root = args
        .path
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let root = root.canonicalize().unwrap_or_else(|_| root.clone());

    let options = BuildOptions {
        include_hidden: args.include_hidden,
        max_file_size_bytes: args.max_file_size_kib.saturating_mul(1024),
        embedding_provider: if args.embedding_provider.trim().is_empty() {
            DEFAULT_EMBEDDING_PROVIDER.to_string()
        } else {
            args.embedding_provider.clone()
        },
        embedding_model: if args.embedding_model.trim().is_empty() {
            DEFAULT_EMBEDDING_MODEL.to_string()
        } else {
            args.embedding_model.clone()
        },
        embedding_dimensions: args.embedding_dimensions.max(64),
        embedding_batch_size: args.embedding_batch_size.max(1),
        embedding_input_chars: args.embedding_input_chars.max(256),
        embedding_max_retries: args.embedding_max_retries,
        embedding_retry_initial_ms: args.embedding_retry_initial_ms.max(1),
        embedding_retry_max_ms: args
            .embedding_retry_max_ms
            .max(args.embedding_retry_initial_ms.max(1)),
    };

    let index = build_index(&root, &options).await?;
    let output_path = args.output.unwrap_or_else(|| default_index_path(&root));

    if let Some(parent) = output_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let encoded = serde_json::to_string_pretty(&index)?;
    tokio::fs::write(&output_path, encoded).await?;

    if args.json {
        let payload = serde_json::json!({
            "index_path": output_path,
            "root": index.root,
            "generated_at": index.generated_at,
            "embedding_provider": index.embedding_provider,
            "embedding_model": index.embedding_model,
            "stats": index.stats,
        });
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        println!("# Codebase Index Built\n");
        println!("- Root: {}", index.root);
        println!("- Output: {}", output_path.display());
        println!(
            "- Embeddings: {}/{}",
            index.embedding_provider, index.embedding_model
        );
        println!("- Indexed files: {}", index.stats.indexed_files);
        println!("- Embedded files: {}", index.stats.embedded_files);
        println!(
            "- Embedding dimensions: {}",
            index.stats.embedding_dimensions
        );
        println!("- Total lines: {}", index.stats.total_lines);
        println!("- Total bytes: {}", index.stats.total_bytes);
        if !index.stats.language_counts.is_empty() {
            let mut langs: Vec<_> = index.stats.language_counts.iter().collect();
            langs.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
            println!("\nTop languages:");
            for (lang, count) in langs.into_iter().take(10) {
                println!("- {}: {} files", lang, count);
            }
        }
    }

    Ok(())
}

pub async fn build_index(root: &Path, options: &BuildOptions) -> Result<CodebaseIndex> {
    let mut stats = IndexStats::default();
    let mut files = Vec::new();
    let mut embedding_inputs = Vec::new();

    let walker = WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| should_descend(entry, options.include_hidden));

    for entry in walker.filter_map(std::result::Result::ok) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        stats.total_seen_files += 1;

        if !options.include_hidden && is_hidden_path(path) {
            stats.skipped_hidden += 1;
            continue;
        }

        let metadata = match std::fs::metadata(path) {
            Ok(meta) => meta,
            Err(_) => {
                stats.skipped_io_errors += 1;
                continue;
            }
        };

        if metadata.len() > options.max_file_size_bytes {
            stats.skipped_large += 1;
            continue;
        }

        if !is_probably_text_file(path) {
            stats.skipped_non_text += 1;
            continue;
        }

        let content = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(_) => {
                stats.skipped_non_text += 1;
                continue;
            }
        };

        let language = detect_language(path);
        let lines = if content.is_empty() {
            0
        } else {
            (content.as_bytes().iter().filter(|b| **b == b'\n').count() + 1) as u32
        };
        let symbol_hints = estimate_symbol_hints(path, &content);

        let rel_path = path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        let modified_unix_ms = metadata
            .modified()
            .ok()
            .and_then(|ts| ts.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|dur| dur.as_millis() as i64);

        files.push(IndexedFile {
            path: rel_path.clone(),
            language: language.clone(),
            bytes: metadata.len(),
            lines,
            symbol_hints,
            modified_unix_ms,
            embedding: Vec::new(),
        });
        embedding_inputs.push(build_embedding_input(
            &rel_path,
            &language,
            &content,
            options.embedding_input_chars,
        ));

        stats.indexed_files += 1;
        stats.total_bytes += metadata.len();
        stats.total_lines += u64::from(lines);
        *stats.language_counts.entry(language).or_insert(0) += 1;
    }

    let backend = resolve_embedding_backend(options).await?;
    let batch_size = options.embedding_batch_size.max(1);
    stats.embedding_dimensions = options.embedding_dimensions.max(64) as u32;

    for start in (0..embedding_inputs.len()).step_by(batch_size) {
        let end = (start + batch_size).min(embedding_inputs.len());
        let embedding_slice = &embedding_inputs[start..end];
        let (vectors, usage) = match &backend {
            EmbeddingBackend::Local { engine, .. } => {
                let vectors = engine.embed_batch(embedding_slice);
                let mut local_prompt_tokens = 0u64;
                let mut local_total_tokens = 0u64;
                for input in embedding_slice {
                    let approx_tokens = approximate_token_count(input);
                    local_prompt_tokens += approx_tokens;
                    local_total_tokens += approx_tokens;
                }
                (vectors, (local_prompt_tokens, local_total_tokens))
            }
            EmbeddingBackend::Remote(engine) => {
                let response = engine.embed_batch(embedding_slice).await.with_context(|| {
                    format!(
                        "failed embedding batch {}-{} via provider {}/{}",
                        start, end, engine.provider_name, engine.model
                    )
                })?;

                let vectors = response.embeddings;
                let prompt_tokens = response.usage.prompt_tokens as u64;
                let total_tokens = response.usage.total_tokens as u64;
                (vectors, (prompt_tokens, total_tokens))
            }
        };

        stats.embedding_prompt_tokens += usage.0;
        stats.embedding_total_tokens += usage.1;

        for (offset, vector) in vectors.into_iter().enumerate() {
            let dim = vector.len() as u32;
            if dim != stats.embedding_dimensions {
                anyhow::bail!(
                    "embedding dimension mismatch: expected {}, got {} (provider: {}, model: {})",
                    stats.embedding_dimensions,
                    dim,
                    backend.provider_name(),
                    backend.model_name(),
                );
            }

            files[start + offset].embedding = vector;
            stats.embedded_files += 1;
        }
    }

    files.sort_by(|a, b| a.path.cmp(&b.path));

    Ok(CodebaseIndex {
        version: INDEX_VERSION,
        root: root.display().to_string(),
        generated_at: Utc::now(),
        embedding_provider: backend.provider_name().to_string(),
        embedding_model: backend.model_name().to_string(),
        stats,
        files,
    })
}

enum EmbeddingBackend {
    Local {
        engine: LocalEmbeddingEngine,
        model: String,
    },
    Remote(RemoteEmbeddingEngine),
}

impl EmbeddingBackend {
    fn provider_name(&self) -> &str {
        match self {
            Self::Local { .. } => LOCAL_EMBEDDING_PROVIDER,
            Self::Remote(engine) => &engine.provider_name,
        }
    }

    fn model_name(&self) -> &str {
        match self {
            Self::Local { model, .. } => model,
            Self::Remote(engine) => &engine.model,
        }
    }
}

#[derive(Clone)]
struct RemoteEmbeddingEngine {
    provider: Arc<dyn crate::provider::Provider>,
    provider_name: String,
    model: String,
    max_retries: u32,
    retry_initial: Duration,
    retry_max: Duration,
}

impl RemoteEmbeddingEngine {
    async fn embed_batch(&self, inputs: &[String]) -> Result<crate::provider::EmbeddingResponse> {
        if inputs.is_empty() {
            return Ok(crate::provider::EmbeddingResponse {
                embeddings: Vec::new(),
                usage: crate::provider::Usage::default(),
            });
        }

        let mut attempt = 0u32;
        loop {
            let request = crate::provider::EmbeddingRequest {
                model: self.model.clone(),
                inputs: inputs.to_vec(),
            };

            match self.provider.embed(request).await {
                Ok(response) => return Ok(response),
                Err(err) => {
                    let should_retry =
                        attempt < self.max_retries && is_retryable_embedding_error(&err);
                    if !should_retry {
                        return Err(anyhow::anyhow!(
                            "remote embedding failed via {}/{} after {} attempt(s): {}",
                            self.provider_name,
                            self.model,
                            attempt + 1,
                            err
                        ));
                    }

                    let delay = retry_delay(attempt, self.retry_initial, self.retry_max);
                    tracing::warn!(
                        provider = %self.provider_name,
                        model = %self.model,
                        attempt = attempt + 1,
                        retry_in_ms = delay.as_millis(),
                        error = %err,
                        "Embedding batch failed, retrying"
                    );

                    tokio::time::sleep(delay).await;
                    attempt += 1;
                }
            }
        }
    }
}

async fn resolve_embedding_backend(options: &BuildOptions) -> Result<EmbeddingBackend> {
    let dimensions = options.embedding_dimensions.max(64);
    if is_local_embedding_provider(&options.embedding_provider) {
        return Ok(EmbeddingBackend::Local {
            engine: LocalEmbeddingEngine::new(dimensions),
            model: options.embedding_model.clone(),
        });
    }

    let model_selector =
        build_model_selector(&options.embedding_provider, &options.embedding_model)?;
    let registry = crate::provider::ProviderRegistry::from_vault().await?;
    let (provider, model) = registry
        .resolve_model(&model_selector)
        .with_context(|| format!("failed resolving embedding model '{model_selector}'"))?;

    let retry_initial = Duration::from_millis(options.embedding_retry_initial_ms.max(1));
    let retry_max = Duration::from_millis(options.embedding_retry_max_ms.max(1));

    Ok(EmbeddingBackend::Remote(RemoteEmbeddingEngine {
        provider_name: provider.name().to_string(),
        provider,
        model,
        max_retries: options.embedding_max_retries,
        retry_initial,
        retry_max,
    }))
}

fn is_local_embedding_provider(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "local" | "hash" | "hash-embedding" | "local/hash-embedding"
    )
}

fn build_model_selector(provider: &str, model: &str) -> Result<String> {
    let provider = provider.trim();
    let model = model.trim();

    if model.is_empty() {
        anyhow::bail!("embedding model cannot be empty");
    }

    if model.contains('/') {
        return Ok(model.to_string());
    }

    if provider.is_empty() {
        anyhow::bail!(
            "embedding provider cannot be empty when model does not include provider prefix"
        );
    }

    Ok(format!("{provider}/{model}"))
}

fn retry_delay(attempt: u32, initial: Duration, max: Duration) -> Duration {
    let multiplier = 2u128.saturating_pow(attempt);
    let initial_ms = initial.as_millis();
    let max_ms = max.as_millis().max(initial_ms);
    let delay_ms = initial_ms.saturating_mul(multiplier).min(max_ms);
    Duration::from_millis(delay_ms as u64)
}

fn is_retryable_embedding_error(error: &anyhow::Error) -> bool {
    let msg = error.to_string().to_ascii_lowercase();
    [
        "timeout",
        "timed out",
        "connection reset",
        "connection refused",
        "temporary",
        "temporarily unavailable",
        "rate limit",
        "too many requests",
        " 429",
        " 500",
        " 502",
        " 503",
        " 504",
    ]
    .iter()
    .any(|needle| msg.contains(needle))
}

fn approximate_token_count(text: &str) -> u64 {
    let words = text.split_whitespace().count() as u64;
    words.max(1)
}

fn build_embedding_input(path: &str, language: &str, content: &str, max_chars: usize) -> String {
    let snippet = safe_char_prefix(content, max_chars);
    format!("path:{path}\nlanguage:{language}\n\n{snippet}")
}

fn safe_char_prefix(input: &str, max_chars: usize) -> String {
    input.chars().take(max_chars).collect()
}

#[derive(Debug, Clone)]
struct LocalEmbeddingEngine {
    dimensions: usize,
}

impl LocalEmbeddingEngine {
    fn new(dimensions: usize) -> Self {
        Self { dimensions }
    }

    fn embed_batch(&self, inputs: &[String]) -> Vec<Vec<f32>> {
        inputs
            .iter()
            .map(|input| self.embed_single(input))
            .collect()
    }

    fn embed_single(&self, input: &str) -> Vec<f32> {
        let mut vector = vec![0.0f32; self.dimensions];
        let tokens = tokenize_for_embedding(input);

        if tokens.is_empty() {
            self.accumulate_char_ngrams(&mut vector, input);
        } else {
            for (idx, token) in tokens.iter().enumerate() {
                let positional_weight = 1.0f32 / (1.0 + (idx as f32 / 128.0));
                self.accumulate_token(&mut vector, token, positional_weight);

                if let Some(next) = tokens.get(idx + 1) {
                    let bigram = format!("{token} {next}");
                    self.accumulate_token(&mut vector, &bigram, positional_weight * 0.65);
                }
            }
        }

        l2_normalize(&mut vector);
        vector
    }

    fn accumulate_char_ngrams(&self, vector: &mut [f32], input: &str) {
        for ngram in input.as_bytes().windows(3).take(2048) {
            let key = String::from_utf8_lossy(ngram);
            self.accumulate_token(vector, &key, 0.5);
        }
    }

    fn accumulate_token(&self, vector: &mut [f32], token: &str, weight: f32) {
        if token.is_empty() {
            return;
        }

        let digest = Sha256::digest(token.as_bytes());
        let len = vector.len();

        let idx_a = (u16::from_le_bytes([digest[0], digest[1]]) as usize) % len;
        let idx_b = (u16::from_le_bytes([digest[2], digest[3]]) as usize) % len;
        let idx_c = (u16::from_le_bytes([digest[4], digest[5]]) as usize) % len;

        let sign_a = if digest[6] & 1 == 0 { 1.0 } else { -1.0 };
        let sign_b = if digest[7] & 1 == 0 { 1.0 } else { -1.0 };
        let sign_c = if digest[8] & 1 == 0 { 1.0 } else { -1.0 };

        vector[idx_a] += sign_a * weight;
        vector[idx_b] += sign_b * (weight * 0.7);
        vector[idx_c] += sign_c * (weight * 0.4);
    }
}

fn tokenize_for_embedding(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            current.push(ch.to_ascii_lowercase());
        } else if !current.is_empty() {
            tokens.push(std::mem::take(&mut current));
            if tokens.len() >= 4096 {
                return tokens;
            }
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}

fn l2_normalize(values: &mut [f32]) {
    let norm = values.iter().map(|v| v * v).sum::<f32>().sqrt();
    if norm > 0.0 {
        for value in values {
            *value /= norm;
        }
    }
}

fn default_index_path(root: &Path) -> PathBuf {
    let mut hasher = Sha256::new();
    hasher.update(root.to_string_lossy().as_bytes());
    let digest = hasher.finalize();
    let short = hex::encode(digest);
    let short = &short[..16];

    let base = crate::config::Config::data_dir().unwrap_or_else(|| root.join(".codetether-agent"));
    base.join("indexes")
        .join(format!("codebase-index-{short}.json"))
}

fn should_descend(entry: &DirEntry, include_hidden: bool) -> bool {
    let path = entry.path();

    if !include_hidden && is_hidden_path(path) {
        return false;
    }

    let skip_dirs = [
        ".git",
        ".hg",
        ".svn",
        "node_modules",
        "target",
        "dist",
        "build",
        ".next",
        "vendor",
        "__pycache__",
        ".venv",
        ".codetether-agent",
    ];

    !path
        .components()
        .any(|c| skip_dirs.contains(&c.as_os_str().to_str().unwrap_or("")))
}

fn is_hidden_path(path: &Path) -> bool {
    path.components().any(|c| {
        c.as_os_str()
            .to_str()
            .map(|name| name.starts_with('.'))
            .unwrap_or(false)
    })
}

fn is_probably_text_file(path: &Path) -> bool {
    let text_exts = [
        "rs", "ts", "js", "tsx", "jsx", "py", "go", "java", "kt", "c", "cpp", "h", "hpp", "md",
        "txt", "json", "yaml", "yml", "toml", "sh", "bash", "zsh", "html", "css", "scss", "sql",
        "proto", "xml", "ini", "env", "lock",
    ];

    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if text_exts.contains(&ext) {
            return true;
        }
    }

    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        return matches!(name, "Dockerfile" | "Makefile" | "Jenkinsfile" | "README");
    }

    false
}

fn detect_language(path: &Path) -> String {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    match ext.as_str() {
        "rs" => "rust",
        "ts" | "tsx" => "typescript",
        "js" | "jsx" => "javascript",
        "py" => "python",
        "go" => "go",
        "java" => "java",
        "kt" => "kotlin",
        "c" | "h" => "c",
        "cpp" | "hpp" | "cc" | "cxx" => "cpp",
        "json" => "json",
        "yaml" | "yml" => "yaml",
        "toml" => "toml",
        "md" => "markdown",
        "sh" | "bash" | "zsh" => "shell",
        "proto" => "proto",
        "sql" => "sql",
        "html" => "html",
        "css" | "scss" => "css",
        _ => {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                match name {
                    "Dockerfile" => "dockerfile",
                    "Makefile" => "makefile",
                    "Jenkinsfile" => "groovy",
                    _ => "text",
                }
            } else {
                "text"
            }
        }
    }
    .to_string()
}

fn estimate_symbol_hints(path: &Path, content: &str) -> u32 {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    let mut count = 0u32;
    for line in content.lines().map(str::trim_start) {
        let hit = match ext.as_str() {
            "rs" => {
                line.starts_with("fn ")
                    || line.starts_with("pub fn ")
                    || line.starts_with("struct ")
                    || line.starts_with("enum ")
                    || line.starts_with("trait ")
                    || line.starts_with("impl ")
            }
            "py" => line.starts_with("def ") || line.starts_with("class "),
            "ts" | "tsx" | "js" | "jsx" => {
                line.starts_with("function ")
                    || line.contains("=>")
                    || line.starts_with("class ")
                    || line.starts_with("export function ")
            }
            "go" => line.starts_with("func ") || line.starts_with("type "),
            "java" | "kt" => {
                line.contains(" class ")
                    || line.starts_with("class ")
                    || line.starts_with("interface ")
                    || line.contains(" fun ")
            }
            _ => false,
        };

        if hit {
            count = count.saturating_add(1);
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    #[test]
    fn detects_hidden_paths() {
        assert!(is_hidden_path(Path::new(".git/config")));
        assert!(is_hidden_path(Path::new("src/.cache/file")));
        assert!(!is_hidden_path(Path::new("src/main.rs")));
    }

    #[test]
    fn language_detection_works() {
        assert_eq!(detect_language(Path::new("src/main.rs")), "rust");
        assert_eq!(detect_language(Path::new("app.py")), "python");
        assert_eq!(detect_language(Path::new("Dockerfile")), "dockerfile");
    }

    #[test]
    fn symbol_hint_estimation_works_for_rust() {
        let content = "pub struct A;\nimpl A {}\nfn run() {}\n";
        assert_eq!(estimate_symbol_hints(Path::new("src/lib.rs"), content), 3);
    }

    #[test]
    fn local_embeddings_have_expected_dimensions() {
        let engine = LocalEmbeddingEngine::new(384);
        let vectors = engine.embed_batch(&["fn main() { println!(\"hi\") }".to_string()]);
        assert_eq!(vectors.len(), 1);
        assert_eq!(vectors[0].len(), 384);
    }

    #[test]
    fn embedding_input_prefix_is_char_safe() {
        let input = "✓✓✓hello";
        let prefixed = build_embedding_input("src/main.rs", "rust", input, 2);
        assert!(prefixed.contains("✓✓"));
    }

    #[test]
    fn local_embedding_provider_aliases_are_supported() {
        assert!(is_local_embedding_provider("local"));
        assert!(is_local_embedding_provider("local/hash-embedding"));
        assert!(is_local_embedding_provider("HASH"));
        assert!(!is_local_embedding_provider("huggingface"));
    }

    #[test]
    fn model_selector_uses_explicit_prefix_when_missing() {
        let selector = build_model_selector("huggingface", "BAAI/bge-small-en-v1.5")
            .expect("model selector should build");
        assert_eq!(selector, "BAAI/bge-small-en-v1.5");

        let selector = build_model_selector("huggingface", "text-embedding-3-large")
            .expect("model selector should build");
        assert_eq!(selector, "huggingface/text-embedding-3-large");
    }

    #[test]
    fn retryable_embedding_error_detection_matches_transient_signals() {
        assert!(is_retryable_embedding_error(&anyhow!(
            "HTTP 429 too many requests"
        )));
        assert!(is_retryable_embedding_error(&anyhow!("gateway timeout")));
        assert!(!is_retryable_embedding_error(&anyhow!(
            "invalid embedding model"
        )));
    }
}

use std::process::Command;

#[test]
fn rlm_fails_fast_without_provider() {
    let output = Command::new(env!("CARGO_BIN_EXE_codetether"))
        .args([
            "rlm",
            "Find all functions",
            "--content",
            "fn main() {}",
            "--json",
        ])
        .env("CODETETHER_DISABLE_ENV_FALLBACK", "1")
        .env("VAULT_ADDR", "http://127.0.0.1:1")
        .env("VAULT_TOKEN", "dummy")
        .env_remove("OPENROUTER_API_KEY")
        .env_remove("OPENAI_API_KEY")
        .env_remove("ANTHROPIC_API_KEY")
        .env_remove("GOOGLE_API_KEY")
        .output()
        .expect("run codetether rlm");

    assert!(
        !output.status.success(),
        "expected fail-fast; stdout={}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("No usable RLM provider found"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn rlm_explicit_local_cuda_does_not_fallback() {
    let output = Command::new(env!("CARGO_BIN_EXE_codetether"))
        .args([
            "rlm",
            "Find all functions",
            "--content",
            "fn main() {}",
            "--json",
            "--model",
            "local_cuda/qwen3-coder-next",
        ])
        .env("VAULT_ADDR", "http://127.0.0.1:1")
        .env("VAULT_TOKEN", "dummy")
        .env("OPENROUTER_API_KEY", "dummy-openrouter-key")
        .env("CODETETHER_LOCAL_CUDA", "1")
        .env_remove("LOCAL_CUDA_MODEL_PATH")
        .env_remove("LOCAL_CUDA_TOKENIZER_PATH")
        .env_remove("CODETETHER_LOCAL_CUDA_MODEL_PATH")
        .env_remove("CODETETHER_LOCAL_CUDA_TOKENIZER_PATH")
        .output()
        .expect("run codetether rlm");

    assert!(
        !output.status.success(),
        "expected explicit local_cuda config error; stdout={}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("local_cuda selected explicitly but runtime is not configured")
            || stderr.contains("local_cuda selected explicitly but provider is unavailable"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn rlm_configured_local_cuda_does_not_silently_fallback() {
    let output = Command::new(env!("CARGO_BIN_EXE_codetether"))
        .args([
            "rlm",
            "Find all functions",
            "--content",
            "fn main() {}",
            "--json",
        ])
        .env("VAULT_ADDR", "http://127.0.0.1:1")
        .env("VAULT_TOKEN", "dummy")
        .env("OPENROUTER_API_KEY", "dummy-openrouter-key")
        .env("LOCAL_CUDA_MODEL_PATH", "/tmp/nonexistent-model.gguf")
        .env(
            "LOCAL_CUDA_TOKENIZER_PATH",
            "/tmp/nonexistent-tokenizer.json",
        )
        .output()
        .expect("run codetether rlm");

    assert!(
        !output.status.success(),
        "expected local_cuda availability error; stdout={}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Local CUDA is configured but unavailable"),
        "unexpected stderr: {stderr}"
    );
}

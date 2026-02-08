//! Example: Using the Thinker for model-backed thought generation
//!
//! This example demonstrates how to use the ThinkerClient directly
//! for generating thoughts with either OpenAI-compatible or Candle backends.
//!
//! Run with:
//!   cargo run --example thinker_example
//!
//! Or with custom configuration:
//!   CODETETHER_COGNITION_THINKER_MODEL=llama3.2 \
//!   CODETETHER_COGNITION_THINKER_BASE_URL=http://localhost:11434/v1 \
//!   cargo run --example thinker_example

use anyhow::Result;
use codetether_agent::cognition::{ThinkerBackend, ThinkerClient, ThinkerConfig};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Create a thinker configuration
    // This uses environment variables or defaults
    let config = ThinkerConfig {
        enabled: true,
        backend: ThinkerBackend::OpenAICompat,
        endpoint: std::env::var("CODETETHER_COGNITION_THINKER_BASE_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:11434/v1/chat/completions".to_string()),
        model: std::env::var("CODETETHER_COGNITION_THINKER_MODEL")
            .unwrap_or_else(|_| "qwen2.5:3b-instruct".to_string()),
        api_key: std::env::var("CODETETHER_COGNITION_THINKER_API_KEY").ok(),
        temperature: 0.2,
        top_p: None,
        max_tokens: 256,
        timeout_ms: 12_000,
        // Candle-specific options (not used with OpenAI backend)
        candle_model_path: None,
        candle_tokenizer_path: None,
        candle_arch: None,
        candle_device: codetether_agent::cognition::CandleDevicePreference::Auto,
        candle_cuda_ordinal: 0,
        candle_repeat_penalty: 1.1,
        candle_repeat_last_n: 64,
        candle_seed: 42,
    };

    println!("Creating thinker client...");
    println!("  Backend: {:?}", config.backend);
    println!("  Model: {}", config.model);
    println!("  Endpoint: {}", config.endpoint);

    // Create the thinker client
    let thinker = ThinkerClient::new(config)?;

    // Define prompts for thought generation
    let system_prompt = "You are a helpful coding assistant. Provide concise, actionable advice.";
    let user_prompt = "What are the key considerations when designing a robust error handling strategy in Rust?";

    println!("\nGenerating thought...");
    println!("  System: {}", system_prompt);
    println!("  User: {}\n", user_prompt);

    // Generate a thought
    match thinker.think(system_prompt, user_prompt).await {
        Ok(output) => {
            println!("✓ Thought generated successfully!");
            println!("  Model: {}", output.model);
            println!("  Finish reason: {:?}", output.finish_reason);
            println!("  Prompt tokens: {:?}", output.prompt_tokens);
            println!("  Completion tokens: {:?}", output.completion_tokens);
            println!("  Total tokens: {:?}", output.total_tokens);
            println!("\nGenerated thought:\n{}", output.text);
        }
        Err(e) => {
            eprintln!("✗ Error generating thought: {}", e);
            std::process::exit(1);
        }
    }

    // Example: Using the thinker for code review
    println!("\n---\n");
    let code_review_system = "You are a code reviewer. Focus on safety, performance, and maintainability.";
    let code_to_review = r#"
fn process_data(data: &str) -> String {
    let mut result = String::new();
    for c in data.chars() {
        if c.is_numeric() {
            result.push(c);
        }
    }
    result
}
"#;

    println!("Performing code review...");
    match thinker.think(code_review_system, code_to_review).await {
        Ok(output) => {
            println!("✓ Code review completed!");
            println!("\nReview:\n{}", output.text);
        }
        Err(e) => {
            eprintln!("✗ Error during code review: {}", e);
        }
    }

    Ok(())
}

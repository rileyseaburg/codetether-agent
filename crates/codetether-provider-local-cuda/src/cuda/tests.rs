//! Existing provider configuration and prompt-safety regression tests.

use super::LocalCudaProvider as GenericProvider;
use super::*;
type LocalCudaProvider = GenericProvider<TestRuntime>;
struct TestRuntime;
impl Runtime for TestRuntime {
    fn new(_: ThinkerConfig) -> Result<Self> {
        Err(anyhow!(
            "unexpected runtime construction in configuration test"
        ))
    }
    async fn think_as_provider(
        &self,
        _: &str,
        _: &str,
        _: &str,
    ) -> Result<codetether_cognition_core::ThinkerOutput> {
        Err(anyhow!("unexpected inference in configuration test"))
    }
}

#[test]
fn test_strip_inline_think_blocks() {
    let raw = "<think>internal plan</think>\n\nHello! How can I help?";
    let cleaned = LocalCudaProvider::strip_inline_think_blocks(raw);
    assert_eq!(cleaned, "Hello! How can I help?");

    let raw_multiple = "before <think>a</think> middle <think>b</think> after";
    let cleaned_multiple = LocalCudaProvider::strip_inline_think_blocks(raw_multiple);
    assert_eq!(cleaned_multiple, "before  middle  after");

    let raw_unclosed = "<think>partial chain";
    let cleaned_unclosed = LocalCudaProvider::strip_inline_think_blocks(raw_unclosed);
    assert!(cleaned_unclosed.is_empty());
}

#[test]
fn test_local_cuda_provider_creation() {
    let provider = LocalCudaProvider::new("test-model".to_string());
    if cfg!(feature = "candle-cuda") {
        if let Ok(p) = provider {
            assert_eq!(p.name(), "local_cuda");
        } else {
            let msg = provider.err().unwrap().to_string();
            assert!(msg.contains("CUDA device unavailable"));
        }
    } else {
        let msg = provider.err().unwrap().to_string();
        assert!(msg.contains("requires a CUDA-enabled build"));
    }
}

#[test]
fn test_cuda_availability_check() {
    // This will return whatever the system has
    let _ = LocalCudaProvider::is_cuda_available();
}

#[test]
fn test_format_messages() {
    let messages = vec![
        Message {
            role: Role::System,
            content: vec![ContentPart::Text {
                text: "You are a helpful assistant.".to_string(),
            }],
        },
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "Hello!".to_string(),
            }],
        },
    ];

    let formatted = LocalCudaProvider::format_messages(&messages);
    assert!(formatted.contains("You are a helpful assistant."));
    assert!(formatted.contains("Hello!"));
}

#[tokio::test]
async fn test_complete_error_message_no_prompt_exposure() {
    let provider = match LocalCudaProvider::new("test-model".to_string()) {
        Ok(p) => p,
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("requires a CUDA-enabled build")
                    || msg.contains("CUDA device unavailable")
            );
            return;
        }
    };

    let request = CompletionRequest {
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "This is a sensitive prompt that should not appear in error messages!"
                    .to_string(),
            }],
        }],
        model: "test-model".to_string(),
        tools: vec![],
        temperature: None,
        top_p: None,
        max_tokens: Some(100),
        stop: vec![],
    };

    let result = provider.complete(request).await;
    let error_message = match result {
        Ok(_) => panic!("Expected local_cuda complete() to fail without model/tokenizer paths"),
        Err(e) => e.to_string(),
    };

    // Verify error message does not contain user prompt content
    assert!(!error_message.contains("sensitive prompt"));
    assert!(!error_message.contains("should not appear"));

    // With no local model configured, provider should return config error.
    assert!(error_message.contains("Local CUDA requires model path"));
}

#[tokio::test]
async fn test_complete_stream_error_message_no_prompt_exposure() {
    let provider = match LocalCudaProvider::new("test-model".to_string()) {
        Ok(p) => p,
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("requires a CUDA-enabled build")
                    || msg.contains("CUDA device unavailable")
            );
            return;
        }
    };

    let request = CompletionRequest {
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "Another sensitive prompt for streaming test".to_string(),
            }],
        }],
        model: "test-model".to_string(),
        tools: vec![],
        temperature: None,
        top_p: None,
        max_tokens: Some(100),
        stop: vec![],
    };

    let result = provider.complete_stream(request).await;
    let error_message = match result {
        Ok(_) => {
            panic!("Expected local_cuda complete_stream() to fail (streaming unimplemented)")
        }
        Err(e) => e.to_string(),
    };

    // Verify error message does not contain user prompt content
    assert!(!error_message.contains("sensitive prompt"));
    assert!(!error_message.contains("streaming test"));

    // Streaming remains unimplemented.
    assert!(
        error_message.contains("not yet implemented") || error_message.contains("not implemented")
    );
}

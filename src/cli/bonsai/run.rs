//! Native CLI execution without provider discovery or an agent loop.
use crate::provider::bonsai::{BonsaiConfig, BonsaiProvider};
use crate::provider::{Provider, StreamChunk};
use anyhow::{Result, ensure};
use futures::StreamExt;
use std::io::Write;
/// Run requests directly, streaming text and printing separate phase timings to stderr.
/// # Arguments
/// * `args` — Prompt and generation options.
/// # Returns
/// Returns after generation or a native-runtime error; no remote fallback is attempted.
/// # Errors
/// Propagates missing-feature, checkpoint, CUDA, sampling, or output errors.
/// # Examples
/// ```text
/// codetether bonsai --prompt "Hello" --repeat 2
/// ```
pub async fn execute(args: super::BonsaiArgs) -> Result<()> {
    ensure!((1..=10).contains(&args.repeat), "repeat must be 1..=10");
    let provider = BonsaiProvider::new(BonsaiConfig::from_environment()?)?;
    for run in 1..=args.repeat {
        let request = super::request::from_args(&args);
        let mut stream = provider.complete_stream(request).await?;
        let mut done = false;
        while let Some(chunk) = stream.next().await {
            match chunk {
                StreamChunk::Text(text) => {
                    print!("{text}");
                    std::io::stdout().flush()?;
                }
                StreamChunk::Done { .. } => {
                    done = true;
                    break;
                }
                StreamChunk::Error(error) => anyhow::bail!(error),
                StreamChunk::KeepAlive => {}
                other => anyhow::bail!("Unexpected Bonsai event: {other:?}"),
            }
        }
        ensure!(done, "Bonsai stream ended without completion");
        println!();
        if let Some(timing) = provider.last_timing() {
            eprintln!(
                "{}",
                serde_json::json!({"run":run,"timing":timing,"decode_tps":timing.decode_tps()})
            );
        }
    }
    Ok(())
}

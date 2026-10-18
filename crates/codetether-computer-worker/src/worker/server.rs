//! Child-side protocol loop: direct platform dispatch, never recursive tool routing.
use super::{codec, framing};
use codetether_computer_contract::input::ComputerUseInput;
use codetether_tool_core::ToolResult;
use tokio::io::{AsyncWriteExt, BufReader};

pub(super) async fn serve<F, Fut>(mut dispatch: F) -> anyhow::Result<()>
where
    F: FnMut(ComputerUseInput) -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<ToolResult>>,
{
    let mut input = BufReader::new(tokio::io::stdin());
    let mut output = tokio::io::stdout();
    while let Some(frame) = framing::read_frame(&mut input, framing::REQUEST_LIMIT).await? {
        let request: ComputerUseInput = serde_json::from_slice(&frame)?;
        let result = match dispatch(request).await {
            Ok(result) => result,
            Err(error) => ToolResult::error(format!("Computer-use action failed: {error:#}")),
        };
        let response = codec::encode(&result, framing::RESPONSE_LIMIT)?;
        output.write_all(&response).await?;
        output.write_all(b"\n").await?;
        output.flush().await?;
    }
    Ok(())
}

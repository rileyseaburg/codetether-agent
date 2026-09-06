//! Child-side protocol loop: direct platform dispatch, never recursive tool routing.
use super::{codec, framing};
use crate::tool::{
    ToolResult,
    computer_use::{input::ComputerUseInput, platform},
};
use tokio::io::{AsyncWriteExt, BufReader};

pub(super) async fn serve() -> anyhow::Result<()> {
    let mut input = BufReader::new(tokio::io::stdin());
    let mut output = tokio::io::stdout();
    while let Some(frame) = framing::read_frame(&mut input, framing::REQUEST_LIMIT).await? {
        let request: ComputerUseInput = serde_json::from_slice(&frame)?;
        let result = match platform::dispatch(&request).await {
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

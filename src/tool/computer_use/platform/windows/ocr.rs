//! Native WinRT OCR entry points; only owned Rust data leaves the blocking thread.

mod angle;
mod apartment;
mod bitmap;
mod capture;
mod dpi;
mod engine;
mod geometry;
mod identity;
mod language;
mod output;
mod probe;
mod recognition;
mod source;
mod status;
#[cfg(test)]
mod tests;

use crate::tool::{ToolResult, computer_use::input::ComputerUseInput};

/// Recognize a file, window, or desktop without blocking a Tokio worker.
///
/// Errors report capture, decoding, apartment, language, or recognition failures.
pub(super) async fn recognize(input: &ComputerUseInput) -> anyhow::Result<ToolResult> {
    source::validate(input.ocr.path.as_deref(), input.hwnd)?;
    input.ocr.validate(input.hwnd)?;
    identity::require()?;
    let (path, language, hwnd) = (
        input.ocr.path.clone(),
        input.ocr.language.clone(),
        input.hwnd,
    );
    tokio::task::spawn_blocking(move || {
        let _apartment = apartment::Apartment::enter()?;
        recognition::run(path.as_deref(), language.as_deref(), hwnd)
    })
    .await?
}

/// Report actual WinRT runtime and installed recognizer availability.
///
/// Errors indicate a blocking-task failure; unavailable runtimes are structured output.
pub(super) async fn status() -> anyhow::Result<ToolResult> {
    tokio::task::spawn_blocking(status::report)
        .await
        .map_err(Into::into)
}
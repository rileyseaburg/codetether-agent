//! Automatic capture and manual callback collection.
use crate::{
    capture_oauth_callback_auto, extract_oauth_code_and_state, prompt_line, prompt_optional_line,
};
use anyhow::Result;
use tokio::time::Duration;

pub(super) async fn read(
    callback_timeout: Duration,
    is_ssh_session: bool,
) -> Result<Option<(String, String)>> {
    let auto_callback = capture_oauth_callback_auto(callback_timeout).await?;
    let callback = if let Some(callback) = auto_callback {
        println!("Captured callback automatically.");
        callback
    } else {
        if is_ssh_session {
            println!(
                "Press Enter to switch to device-code auth, or paste callback URL from your browser."
            );
        }
        let callback_input = if is_ssh_session {
            prompt_optional_line("Callback URL: ")?
        } else {
            prompt_line("Callback URL: ")?
        };

        if callback_input.trim().is_empty() {
            return Ok(None);
        }

        extract_oauth_code_and_state(&callback_input)?
    };

    Ok(Some(callback))
}

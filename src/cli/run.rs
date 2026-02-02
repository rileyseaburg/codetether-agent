//! Non-interactive run command

use super::RunArgs;
use crate::session::Session;
use anyhow::Result;

pub async fn execute(args: RunArgs) -> Result<()> {
    let message = args.message.trim();
    
    if message.is_empty() {
        anyhow::bail!("You must provide a message");
    }

    tracing::info!("Running with message: {}", message);

    // Create or continue session
    let mut session = if let Some(session_id) = args.session {
        Session::load(&session_id).await?
    } else if args.continue_session {
        Session::last().await?
    } else {
        Session::new().await?
    };

    // Set model if specified
    if let Some(model) = args.model {
        tracing::info!("Using specified model: {}", model);
        session.metadata.model = Some(model);
    }

    // Execute the prompt
    let result = session.prompt(message).await?;

    // Output based on format
    match args.format.as_str() {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        _ => {
            println!("{}", result.text);
        }
    }

    Ok(())
}

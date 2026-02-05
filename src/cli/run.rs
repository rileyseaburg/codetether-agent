//! Non-interactive run command

use super::RunArgs;
use crate::config::Config;
use crate::session::Session;
use anyhow::Result;

pub async fn execute(args: RunArgs) -> Result<()> {
    let message = args.message.trim();

    if message.is_empty() {
        anyhow::bail!("You must provide a message");
    }

    tracing::info!("Running with message: {}", message);

    // Load configuration
    let config = Config::load().await.unwrap_or_default();

    // Create or continue session - default is to continue last session if it exists
    let mut session = if let Some(session_id) = args.session {
        tracing::info!("Continuing session: {}", session_id);
        Session::load(&session_id).await?
    } else if args.continue_session {
        match Session::last().await {
            Ok(s) => {
                tracing::info!("Continuing last session: {}", s.id);
                s
            }
            Err(_) => {
                let s = Session::new().await?;
                tracing::info!("Created new session: {}", s.id);
                s
            }
        }
    } else {
        let s = Session::new().await?;
        tracing::info!("Created new session: {}", s.id);
        s
    };

    // Set model: CLI arg > env var > config default
    let model = args
        .model
        .or_else(|| std::env::var("CODETETHER_DEFAULT_MODEL").ok())
        .or(config.default_model);

    if let Some(model) = model {
        tracing::info!("Using model: {}", model);
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
            // Show session ID for continuation
            eprintln!(
                "\n[Session: {} | Continue with: codetether run -c \"...\"]",
                session.id
            );
        }
    }

    Ok(())
}

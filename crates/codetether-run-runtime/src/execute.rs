//! Ordered setup and reporting around one ordinary run session.
use crate::{Host, Input, Session};
use codetether_run_resume::SessionSource;
/// Execute a prepared ordinary run with native service bindings.
/// # Arguments
/// * `input` - Prepared CLI arguments, configuration and workspace context.
/// # Returns
/// Unit after live events finish and the final result is printed.
/// # Errors
/// Returns session, policy, prompt, event persistence or output errors.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_run_runtime::Host>(input: codetether_run_runtime::Input) -> anyhow::Result<()> {
/// codetether_run_runtime::execute::<H>(input).await?;
/// # Ok(()) }
/// ```
pub async fn execute<H: Host>(input: Input) -> anyhow::Result<()> {
    let Input {
        args,
        config,
        workspace_dir,
        knowledge_snapshot,
        message,
        jsonl_output,
    } = input;
    let mut session =
        codetether_run_resume::select_session::<H::Session>(&args, &workspace_dir).await?;
    if jsonl_output {
        codetether_run_jsonl::write_started(SessionSource::id(&session))?;
    }

    // Set model: CLI arg > env var > config default
    codetether_run_policy::apply_model(&mut session, &args, config.default_model);

    // Load project memory palace
    codetether_run_context::load_memory(&workspace_dir);

    session.set_knowledge_snapshot(knowledge_snapshot);
    codetether_run_policy::apply_session_policy(&mut session, &args)?;

    // Wire bus for thinking capture + S3 training data
    let bus = H::create_bus();
    session.attach_bus(bus);

    let live_events = codetether_run_events::LiveEvents::<H::Session>::start(
        SessionSource::id(&session),
        &message,
        jsonl_output,
    )
    .await?;

    let result =
        crate::turn::run(&mut session, &args, &message, &workspace_dir, live_events).await?;

    // Output based on format
    codetether_run_report::render_session(
        &args.format,
        &result,
        session.usage(),
        SessionSource::id(&session),
    )?;

    Ok(())
}

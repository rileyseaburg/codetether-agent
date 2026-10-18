//! Start a forage run while retaining ownership of its live services.
use super::*;

/// Run forage using the configured selection and execution options.
///
/// # Arguments
/// `args` supplies goal inputs, execution choices and loop limits.
/// # Returns
/// Completes when the configured run finishes.
/// # Errors
/// Returns setup, discovery or execution errors according to the fail-fast policy.
/// # Examples
/// ```rust,no_run
/// async fn example<H: codetether_forage_runtime::Host>(args: codetether_cli_options::ForageArgs) -> anyhow::Result<()> {
///     let _ = codetether_forage_runtime::execute::<H>(args).await?;
///     Ok(())
/// }
/// ```
pub async fn execute<H: Host>(args: ForageArgs) -> Result<()> {
    execute_with_summary::<H>(args).await.map(|_| ())
}

/// Run forage using the configured selection and execution options.
///
/// # Arguments
/// `args` supplies goal inputs, execution choices and loop limits.
/// # Returns
/// The run counters and final selection.
/// # Errors
/// Returns setup, discovery or execution errors according to the fail-fast policy.
/// # Examples
/// ```rust,no_run
/// async fn example<H: codetether_forage_runtime::Host>(args: codetether_cli_options::ForageArgs) -> anyhow::Result<()> {
///     let _ = codetether_forage_runtime::execute_with_summary::<H>(args).await?;
///     Ok(())
/// }
/// ```
pub async fn execute_with_summary<H: Host>(args: ForageArgs) -> Result<ForageRunSummary> {
    ensure_audit_log_initialized().await;
    let repo = OkrRepository::from_config().await?;
    let moonshot_rubric = load_moonshot_rubric(&args).await?;
    if args.execute {
        seed_initial_okr_if_empty(&repo, &moonshot_rubric).await?;
    }
    let bus = AgentBus::new().into_arc();
    // Make forage observable to other in-process agents/tools: install this
    // bus as the process-global one when none has been set yet (first writer
    // wins, so this is a no-op if an entrypoint already installed a bus).
    codetether_bus_core::global::set_global(bus.clone());
    // S3 archival is best-effort: forage always runs, even without MinIO/S3.
    // `--no-s3` skips the attempt entirely; otherwise we try and degrade to
    // local-only mode (with a warning) when no configuration is present.
    let mut s3_sync_handle = start_sink::start(&args, &bus).await;
    // Only police S3 liveness if a sink actually started.
    let require_s3 = s3_sync_handle.is_some();
    let forage_id = "forage-runtime";
    let bus_handle = bus.handle(forage_id);
    let mut observer = bus.handle("forage-observer");
    let events = events::RunEvents {
        args: &args,
        moonshot_rubric: &moonshot_rubric,
        bus_handle: &bus_handle,
    };
    events.start().await;

    runtime::RunLoop::<H> {
        args: &args,
        repo: &repo,
        moonshot_rubric: &moonshot_rubric,
        events: &events,
        observer: &mut observer,
        s3_sync_handle: &mut s3_sync_handle,
        require_s3,
        host: std::marker::PhantomData,
    }
    .run()
    .await
}

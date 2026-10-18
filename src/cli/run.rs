//! Non-interactive run command

mod files;
mod jsonl;
mod knowledge;
mod relay_host;
mod run_execute;
mod session_runtime;
mod session_source;
mod sol_workflow;
use relay_host::run_protocol_first_relay;

use super::RunArgs;
use crate::autochat::model_rotation::resolve_provider_for_model_autochat;
use crate::okr::{Okr, OkrRepository};
use anyhow::Result;
use codetether_autochat_core::AUTOCHAT_MAX_AGENTS;
use codetether_relay_plan::{
    default_relay_okr_template, okr_from_planned_draft, truncate_with_ellipsis,
};
use codetether_relay_planner::plan_okr_draft_with_registry;
use codetether_run_command::resolve_autochat_model;
use uuid::Uuid;

pub use run_execute::execute;

pub(super) async fn execute_inner(args: RunArgs) -> Result<()> {
    let message = files::with_attachments(args.message.trim(), &args.file)?;
    let jsonl_output = jsonl::enabled(&args.format);
    super::run_checkpoint::validate_auto_continue(args.auto_continue_until)?;

    if message.is_empty() {
        anyhow::bail!("You must provide a message");
    }

    tracing::info!("Running with message: {}", message);

    codetether_swarm_cli::prepare_speculative(&args, &message, jsonl_output)?;

    // Load configuration
    let config = super::run_config::load(&args).await;
    let workspace_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let knowledge_snapshot = knowledge::refresh(&workspace_dir).await;

    // Protocol-first relay aliases in CLI:
    // - /go [count] <task>
    // - /autochat [count] <task>
    if let Some(request) = codetether_run_command::parse_relay_request(&message, jsonl_output)? {
        let codetether_run_command::RelayRequest {
            agent_count,
            task,
            require_prd,
            easy_go_requested,
            explicit_count,
        } = request;

        let model = resolve_autochat_model(
            args.model.as_deref(),
            std::env::var("CODETETHER_DEFAULT_MODEL").ok().as_deref(),
            config.default_model.as_deref(),
            easy_go_requested,
        );

        // PRD-gated execution path (mandatory for /go and default for /autochat)
        if require_prd {
            // For /go, default to max concurrency (run all stories in parallel) unless
            // the user explicitly specified a count. /autochat keeps the requested count.
            let max_concurrent = if easy_go_requested && !explicit_count {
                AUTOCHAT_MAX_AGENTS
            } else {
                agent_count
            };
            // Create OKR draft (LLM-proposed, with safe fallback)
            let okr_id = Uuid::new_v4();
            let registry_for_draft = crate::provider::ProviderRegistry::shared_from_vault()
                .await
                .ok();

            let (mut okr, draft_note) = if let Some(registry) = &registry_for_draft {
                match plan_okr_draft_with_registry(&task, &model, agent_count, registry).await {
                    Some(planned) => (okr_from_planned_draft(okr_id, &task, planned), None),
                    None => (
                        default_relay_okr_template(okr_id, &task),
                        Some("(OKR: fallback template — model draft parse failed)".to_string()),
                    ),
                }
            } else {
                (
                    default_relay_okr_template(okr_id, &task),
                    Some("(OKR: fallback template — provider unavailable)".to_string()),
                )
            };

            // Create run
            let mut run = codetether_relay_okr::cli_approval::new_run(okr_id);

            // Show OKR draft
            if !codetether_relay_okr::cli_approval::ask(
                &mut run,
                codetether_relay_okr::cli_approval::Prompt {
                    easy_go_requested,
                    task_preview: &truncate_with_ellipsis(&task, 80),
                    agent_count,
                    model: &model,
                    draft_note: draft_note.as_deref(),
                    okr: &okr,
                },
            )? {
                return Ok(());
            }

            // Save OKR and run
            let mut approved_run = run;
            codetether_relay_okr::cli_approval::persist(okr_id, &okr, &mut approved_run).await;

            // Load provider for Ralph execution
            let registry = if let Some(registry) = registry_for_draft {
                registry
            } else {
                crate::provider::ProviderRegistry::shared_from_vault().await?
            };
            let (provider, resolved_model) = resolve_provider_for_model_autochat(&registry, &model)
                .ok_or_else(|| anyhow::anyhow!("No provider available for model '{model}'"))?;

            // Wire bus for training data capture
            let bus = crate::bus::AgentBus::new().into_arc();
            crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());

            // Execute via Ralph PRD loop — use max_concurrent as concurrency
            let ralph_result = super::go_ralph::execute_go_ralph(
                &task,
                &mut okr,
                &mut approved_run,
                provider,
                &resolved_model,
                10,                     // max iterations
                Some(bus),              // bus for training data
                max_concurrent,         // max concurrent stories
                Some(registry.clone()), // relay registry
            )
            .await?;

            // Persist final run state
            if let Ok(repo) = OkrRepository::from_config().await {
                let _ = repo.update_run(approved_run).await;
            }

            // Display results
            codetether_run_report::render_ralph(&args.format, &ralph_result, &task)?;
            return Ok(());
        }

        // Explicit tactical /autochat path (requires --no-prd)
        let relay_result = run_protocol_first_relay(agent_count, &task, &model, None, None).await?;
        codetether_run_report::render_relay(&args.format, &relay_result)?;
        return Ok(());
    }

    // Create or continue session.
    codetether_run_runtime::execute::<session_runtime::NativeHost>(codetether_run_runtime::Input {
        args,
        config,
        workspace_dir,
        knowledge_snapshot,
        message,
        jsonl_output,
    })
    .await
}

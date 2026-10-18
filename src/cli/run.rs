//! Non-interactive run command

mod events;
mod files;
mod jsonl;
mod knowledge;
mod relay_host;
mod run_execute;
mod sol_workflow;
use relay_host::run_protocol_first_relay;

use super::RunArgs;
use crate::autochat::model_rotation::resolve_provider_for_model_autochat;
use crate::bus::AgentBus;
use crate::okr::{ApprovalDecision, Okr, OkrRepository, OkrRun};
use crate::session::{Session, import_codex_session_by_id};
use anyhow::Result;
use codetether_autochat_core::AUTOCHAT_MAX_AGENTS;
use codetether_relay_plan::{
    default_relay_okr_template, okr_from_planned_draft, truncate_with_ellipsis,
};
use codetether_relay_planner::plan_okr_draft_with_registry;
use codetether_run_command::{
    command_with_optional_args, is_easy_go_command, normalize_cli_go_command,
    normalize_go_task_input, resolve_autochat_model, validate_easy_go_task,
};
use std::io::Write;
use uuid::Uuid;
const AUTOCHAT_DEFAULT_AGENTS: usize = crate::autochat::AUTOCHAT_DEFAULT_AGENTS;
const AUTOCHAT_QUICK_DEMO_TASK: &str = crate::autochat::AUTOCHAT_QUICK_DEMO_TASK;

pub use run_execute::execute;

pub(super) async fn execute_inner(args: RunArgs) -> Result<()> {
    let message = files::with_attachments(args.message.trim(), &args.file)?;
    let jsonl_output = jsonl::enabled(&args.format);
    super::run_checkpoint::validate_auto_continue(args.auto_continue_until)?;

    if message.is_empty() {
        anyhow::bail!("You must provide a message");
    }

    tracing::info!("Running with message: {}", message);

    if args.branches > 1 {
        let runner = crate::swarm::speculative::SpeculativeRunner::new(
            args.branches,
            args.strategies.clone(),
        );
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let specs = runner.build_branches(&cwd, &message)?;
        tracing::info!(branches = %runner.branch_count, "Many-worlds speculative mode enabled");
        for spec in &specs {
            tracing::info!(
                branch = %spec.branch_name,
                strategy = %spec.strategy_prompt,
                "Speculative branch assigned"
            );
        }
        if !jsonl_output {
            println!(
                "[speculative] {} parallel branches queued (collapse controller will race + prune)",
                runner.branch_count
            );
        }
        // TODO: wire through SwarmExecutor when parallel dispatch is ready
    }

    // Load configuration
    let config = super::run_config::load(&args).await;
    let workspace_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let knowledge_snapshot = knowledge::refresh(&workspace_dir).await;

    // Protocol-first relay aliases in CLI:
    // - /go [count] <task>
    // - /autochat [count] <task>
    let easy_go_requested = is_easy_go_command(&message);
    let normalized = normalize_cli_go_command(&message);
    if let Some(rest) = command_with_optional_args(&normalized, "/autochat") {
        let Some(parsed) = crate::autochat::parse_autochat_request(
            rest,
            AUTOCHAT_DEFAULT_AGENTS,
            AUTOCHAT_QUICK_DEMO_TASK,
        ) else {
            anyhow::bail!(
                "Usage: /autochat [count] [--no-prd] <task>\nEasy mode: /go <task>\ncount range: 2-{} (default: {})",
                AUTOCHAT_MAX_AGENTS,
                AUTOCHAT_DEFAULT_AGENTS
            );
        };
        let agent_count = parsed.agent_count;
        let parsed_task = parsed.task;
        let task = if easy_go_requested {
            validate_easy_go_task(&parsed_task)?
        } else {
            normalize_go_task_input(&parsed_task)
        };
        let require_prd = easy_go_requested || !parsed.bypass_prd;
        if require_prd && jsonl_output {
            anyhow::bail!(
                "--format jsonl does not support interactive /go approval; use /autochat --no-prd"
            );
        }

        if !(2..=AUTOCHAT_MAX_AGENTS).contains(&agent_count) {
            anyhow::bail!(
                "Invalid relay size {}. count must be between 2 and {}",
                agent_count,
                AUTOCHAT_MAX_AGENTS
            );
        }

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
            let max_concurrent = if easy_go_requested && !parsed.explicit_count {
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
            let mut run = OkrRun::new(
                okr_id,
                format!("Run {}", chrono::Local::now().format("%Y-%m-%d %H:%M")),
            );
            let _ = run.submit_for_approval();

            // Show OKR draft
            let command_label = if easy_go_requested {
                "/go"
            } else {
                "/autochat"
            };
            println!("\n⚠️  {command_label} OKR Draft\n");
            println!("Task: {}", truncate_with_ellipsis(&task, 80));
            println!("Agents: {} | Model: {}", agent_count, model);
            if let Some(note) = draft_note {
                println!("{}", note);
            }
            println!("\nObjective: {}", okr.title);
            println!("\nKey Results:");
            for kr in &okr.key_results {
                println!("  • {} (target: {} {})", kr.title, kr.target_value, kr.unit);
            }
            println!("\n");

            // Prompt for approval
            print!("Approve OKR and start relay? [y/n]: ");
            std::io::stdout().flush()?;
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let input = input.trim().to_lowercase();
            if input != "y" && input != "yes" {
                run.record_decision(ApprovalDecision::deny(run.id, "User denied via CLI"));
                println!("❌ OKR denied. Relay not started.");
                println!("Use /autochat --no-prd for tactical execution without OKR/PRD tracking.");
                return Ok(());
            }

            println!("✅ OKR approved! Starting Ralph PRD execution...\n");

            // Save OKR and run
            let mut approved_run = run;
            if let Ok(repo) = OkrRepository::from_config().await {
                let _ = repo.create_okr(okr.clone()).await;
                approved_run.record_decision(ApprovalDecision::approve(
                    approved_run.id,
                    "User approved via CLI",
                ));
                approved_run.correlation_id = Some(format!("ralph-{}", Uuid::new_v4()));
                let _ = repo.create_run(approved_run.clone()).await;
                tracing::info!(okr_id = %okr_id, okr_run_id = %approved_run.id, "OKR run approved and saved");
            }

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
            match args.format.as_str() {
                "jsonl" => {
                    let response = super::go_ralph::format_go_ralph_result(&ralph_result, &task);
                    jsonl::write_completed(None, &response, None)?;
                }
                "json" => println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "passed": ralph_result.passed,
                        "total": ralph_result.total,
                        "all_passed": ralph_result.all_passed,
                        "iterations": ralph_result.iterations,
                        "feature_branch": ralph_result.feature_branch,
                        "prd_path": ralph_result.prd_path.display().to_string(),
                        "status": format!("{:?}", ralph_result.status),
                        "stories": ralph_result.stories.iter().map(|s| serde_json::json!({
                            "id": s.id,
                            "title": s.title,
                            "passed": s.passed,
                        })).collect::<Vec<_>>(),
                    }))?
                ),
                _ => {
                    println!(
                        "{}",
                        super::go_ralph::format_go_ralph_result(&ralph_result, &task)
                    );
                }
            }
            return Ok(());
        }

        // Explicit tactical /autochat path (requires --no-prd)
        let relay_result = run_protocol_first_relay(agent_count, &task, &model, None, None).await?;
        match args.format.as_str() {
            "jsonl" => {
                if let Some(failure) = &relay_result.failure {
                    jsonl::write_failed_response(failure, Some(&relay_result.summary))?;
                } else {
                    jsonl::write_completed(None, &relay_result.summary, None)?;
                }
            }
            "json" => println!("{}", serde_json::to_string_pretty(&relay_result)?),
            _ => {
                println!("{}", relay_result.summary);
                if let Some(failure) = &relay_result.failure {
                    eprintln!("\nFailure detail: {}", failure);
                }
                eprintln!(
                    "\n[Relay: {} | Model: {}]",
                    relay_result.relay_id, relay_result.model
                );
            }
        }
        return Ok(());
    }

    // Create or continue session.
    let mut session = if let Some(codex_id) = args.codex_session.clone() {
        tracing::info!("Importing Codex session: {}", codex_id);
        import_codex_session_by_id(&codex_id).await?
    } else if let Some(session_id) = args.session.clone() {
        tracing::info!("Continuing session: {}", session_id);
        Session::resume(&session_id).await?
    } else if args.continue_session {
        match Session::resume_last(Some(&workspace_dir)).await {
            Ok(s) => {
                tracing::info!(
                    session_id = %s.id,
                    workspace = %workspace_dir.display(),
                    "Continuing last workspace session"
                );
                s
            }
            Err(err) => {
                let s = Session::new().await?;
                tracing::warn!(
                    error = %err,
                    session_id = %s.id,
                    workspace = %workspace_dir.display(),
                    "Session lookup failed; created new session"
                );
                s
            }
        }
    } else {
        let s = Session::new().await?;
        tracing::info!("Created new session: {}", s.id);
        s
    };
    if jsonl_output {
        jsonl::write_started(&session.id)?;
    }

    // Set model: CLI arg > env var > config default
    let model = args
        .model
        .clone()
        .or_else(|| std::env::var("CODETETHER_DEFAULT_MODEL").ok())
        .or(config.default_model);

    if let Some(model) = model {
        tracing::info!("Using model: {}", model);
        session.metadata.model = Some(model);
    }

    // Load project memory palace
    let beliefs = crate::memory::palace::load_project_beliefs(&workspace_dir);
    if !beliefs.is_empty() {
        let ctx = crate::memory::palace::belief_context(&beliefs);
        if !ctx.is_empty() {
            tracing::info!(beliefs = beliefs.len(), "Loaded project memory palace");
        }
    }

    session.metadata.knowledge_snapshot = knowledge_snapshot;
    super::run_config::apply_session_policy(&mut session, &args)?;

    // Wire bus for thinking capture + S3 training data
    let bus = AgentBus::new().into_arc();
    crate::bus::set_global(bus.clone());
    crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());
    session.bus = Some(bus);

    let live_events = events::LiveEvents::start(&session.id, &message, jsonl_output).await?;

    let result = match super::run_loop::execute_prompt_with_resume_events(
        &mut session,
        &message,
        args.max_steps,
        args.auto_continue_until,
        &workspace_dir,
        live_events.sender(),
    )
    .await
    {
        Ok(result) => {
            live_events.finish_success(&result.text).await?;
            result
        }
        Err(error) => {
            let message = error.to_string();
            live_events.finish_failure(&message).await?;
            return Err(error);
        }
    };

    // Output based on format
    match args.format.as_str() {
        "jsonl" => jsonl::write_completed(
            Some(result.session_id.as_str()),
            &result.text,
            jsonl::usage(&session.usage),
        )?,
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

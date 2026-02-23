//! CodeTether Agent - A2A-native AI coding agent
//!
//! A Rust implementation of an AI coding agent with first-class support for the
//! A2A (Agent-to-Agent) protocol and the CodeTether ecosystem.
//!
//! By default, runs as an A2A worker connecting to the CodeTether platform.
//! Use the 'tui' subcommand for interactive terminal mode.
//!
//! SECURITY: All API keys and secrets are loaded exclusively from HashiCorp Vault.
//! Environment variables are NOT used for secrets (only for Vault connection bootstrap).

mod a2a;
mod agent;
mod audit;
mod benchmark;
mod bus;
mod cli;
mod cognition;
mod config;
mod crash;
mod event_stream;
mod k8s;
mod lsp;
pub mod mcp;
mod moltbook;
mod okr;
mod provider;
pub mod ralph;
pub mod rlm;
pub mod secrets;
mod server;
mod session;
pub mod swarm;
pub mod telemetry;
mod tool;
mod tui;
mod worker_server;
mod worktree;

use clap::Parser;
use cli::{Cli, Command};
use std::io::IsTerminal;
use std::sync::Arc;
use swarm::{DecompositionStrategy, ExecutionMode, SwarmExecutor};
use telemetry::{TOKEN_USAGE, get_persistent_stats};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

fn normalize_provider_alias(name: &str) -> &str {
    match name {
        "local-cuda" | "localcuda" => "local_cuda",
        "zhipuai" => "zai",
        other => other,
    }
}

fn local_cuda_runtime_configured() -> bool {
    let model_path = std::env::var("LOCAL_CUDA_MODEL_PATH")
        .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_MODEL_PATH"))
        .ok();
    let tokenizer_path = std::env::var("LOCAL_CUDA_TOKENIZER_PATH")
        .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_TOKENIZER_PATH"))
        .ok();
    model_path.is_some() && tokenizer_path.is_some()
}

fn local_cuda_model_name() -> String {
    std::env::var("LOCAL_CUDA_MODEL")
        .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_MODEL"))
        .unwrap_or_else(|_| "qwen3-coder-next".to_string())
}

fn default_openrouter_rlm_model() -> String {
    std::env::var("CODETETHER_RLM_DEFAULT_MODEL")
        .or_else(|_| std::env::var("OPENROUTER_RLM_MODEL"))
        .unwrap_or_else(|_| "qwen/qwen3.5-coder-7b".to_string())
}

fn resolve_rlm_provider_and_model(
    model_arg: Option<&str>,
    registry: &provider::ProviderRegistry,
) -> anyhow::Result<(Arc<dyn provider::Provider>, String, String)> {
    if let Some(model_ref) = model_arg {
        let (provider_name, model_name) = provider::parse_model_string(model_ref);
        if let Some(provider_name) = provider_name {
            let normalized = normalize_provider_alias(provider_name);
            let provider = registry.get(normalized).ok_or_else(|| {
                anyhow::anyhow!(
                    "Requested provider '{}' is not available. Configure it in Vault or env.",
                    normalized
                )
            })?;

            if normalized == "local_cuda" && !local_cuda_runtime_configured() {
                anyhow::bail!(
                    "local_cuda selected explicitly but runtime is not configured. \
                     Set LOCAL_CUDA_MODEL_PATH and LOCAL_CUDA_TOKENIZER_PATH \
                     (or CODETETHER_LOCAL_CUDA_MODEL_PATH / CODETETHER_LOCAL_CUDA_TOKENIZER_PATH)."
                );
            }

            return Ok((provider, model_name.to_string(), normalized.to_string()));
        }

        // Unqualified model: local_cuda first when usable, then openrouter.
        if local_cuda_runtime_configured()
            && let Some(provider) = registry.get("local_cuda")
        {
            return Ok((provider, model_ref.to_string(), "local_cuda".to_string()));
        }
        if let Some(provider) = registry.get("openrouter") {
            return Ok((provider, model_ref.to_string(), "openrouter".to_string()));
        }
        anyhow::bail!(
            "No provider available for model '{}'. Configure local_cuda or openrouter.",
            model_ref
        );
    }

    // No explicit model: prefer local_cuda when usable, otherwise openrouter.
    if local_cuda_runtime_configured()
        && let Some(provider) = registry.get("local_cuda")
    {
        return Ok((provider, local_cuda_model_name(), "local_cuda".to_string()));
    }

    if let Some(provider) = registry.get("openrouter") {
        return Ok((
            provider,
            default_openrouter_rlm_model(),
            "openrouter".to_string(),
        ));
    }

    anyhow::bail!(
        "No usable RLM provider found. Configure local CUDA \
         (LOCAL_CUDA_MODEL_PATH + LOCAL_CUDA_TOKENIZER_PATH) \
         or OpenRouter credentials."
    );
}

fn gather_rlm_content(args: &cli::RlmArgs) -> anyhow::Result<String> {
    use std::io::Read;

    if !args.file.is_empty() {
        let mut parts = Vec::new();
        for path in &args.file {
            match std::fs::read_to_string(path) {
                Ok(content) => parts.push(format!(
                    "=== FILE: {} ===\n{}\n=== END FILE ===\n",
                    path.display(),
                    content
                )),
                Err(e) => parts.push(format!(
                    "=== FILE: {} ===\nError: {}\n=== END FILE ===\n",
                    path.display(),
                    e
                )),
            }
        }
        return Ok(parts.join("\n"));
    }

    if let Some(ref c) = args.content {
        if c == "-" {
            let mut stdin_content = String::new();
            std::io::stdin().read_to_string(&mut stdin_content)?;
            return Ok(stdin_content);
        }
        return Ok(c.clone());
    }

    anyhow::bail!("Either --file or --content must be provided");
}

fn detect_rlm_content_type(args: &cli::RlmArgs, content: &str) -> rlm::ContentType {
    if args.content_type == "auto" {
        return rlm::RlmChunker::detect_content_type(content);
    }

    match args.content_type.as_str() {
        "code" => rlm::ContentType::Code,
        "logs" => rlm::ContentType::Logs,
        "conversation" => rlm::ContentType::Conversation,
        "documents" => rlm::ContentType::Documents,
        _ => rlm::ContentType::Mixed,
    }
}

fn oracle_status_line(result: &rlm::OracleResult) -> String {
    match result {
        rlm::OracleResult::Golden(_) => {
            "[oracle: golden ✓] deterministic verification passed".to_string()
        }
        rlm::OracleResult::Consensus {
            agreement_ratio, ..
        } => format!(
            "[oracle: consensus ✓] semantic agreement {:.1}%",
            agreement_ratio * 100.0
        ),
        rlm::OracleResult::Unverified { reason, .. } => {
            format!("[oracle: unverified —] {}", reason)
        }
        rlm::OracleResult::Failed { reason, .. } => format!("[oracle: failed ✗] {}", reason),
    }
}

fn oracle_json_value(
    result: &rlm::OracleResult,
    split: Option<rlm::oracle::SplitWriteStats>,
    persist: Option<rlm::OracleTracePersistResult>,
) -> serde_json::Value {
    let base = match result {
        rlm::OracleResult::Golden(trace) => serde_json::json!({
            "status": "golden",
            "verification_method": format!("{:?}", trace.verification_method),
            "verdict": trace.verdict,
            "trace_id": trace.trace_id,
        }),
        rlm::OracleResult::Consensus {
            trace,
            agreement_ratio,
        } => serde_json::json!({
            "status": "consensus",
            "agreement_ratio": agreement_ratio,
            "verification_method": format!("{:?}", trace.verification_method),
            "verdict": trace.verdict,
            "trace_id": trace.trace_id,
        }),
        rlm::OracleResult::Unverified { reason, trace } => serde_json::json!({
            "status": "unverified",
            "reason": reason,
            "verification_method": format!("{:?}", trace.verification_method),
            "verdict": trace.verdict,
            "trace_id": trace.trace_id,
        }),
        rlm::OracleResult::Failed {
            reason,
            diff,
            trace,
        } => serde_json::json!({
            "status": "failed",
            "reason": reason,
            "diff": diff,
            "verification_method": format!("{:?}", trace.verification_method),
            "verdict": trace.verdict,
            "trace_id": trace.trace_id,
        }),
    };

    let mut object = base.as_object().cloned().unwrap_or_default();
    object.insert(
        "split".to_string(),
        serde_json::to_value(split).unwrap_or_default(),
    );
    object.insert(
        "persist".to_string(),
        serde_json::to_value(persist).unwrap_or_default(),
    );
    serde_json::Value::Object(object)
}

async fn run_rlm_command(args: cli::RlmArgs) -> anyhow::Result<()> {
    use provider::ProviderRegistry;

    if args.consensus_runs == 0 {
        anyhow::bail!("--consensus-runs must be at least 1");
    }
    if !(0.0..=1.0).contains(&args.consensus_threshold) {
        anyhow::bail!("--consensus-threshold must be between 0.0 and 1.0");
    }
    if args.oracle_verify {
        eprintln!(
            "warning: --oracle-verify is deprecated, oracle verification is now always-on. \
             Use --no-oracle-verify to disable."
        );
    }

    let content = gather_rlm_content(&args)?;
    let input_tokens = rlm::RlmChunker::estimate_tokens(&content);
    let content_type = detect_rlm_content_type(&args, &content);
    tracing::info!(input_tokens, "RLM: Analyzing content");

    let registry = ProviderRegistry::from_vault().await.map_err(|e| {
        anyhow::anyhow!(
            "Failed to load providers for Oracle-First RLM: {}. \
             Configure local_cuda or openrouter.",
            e
        )
    })?;
    let (provider, model, provider_name) =
        resolve_rlm_provider_and_model(args.model.as_deref(), &registry)?;
    tracing::info!(provider = %provider_name, model = %model, "RLM provider resolved");

    let run_count = args.consensus_runs;
    let run_temperature = args
        .analysis_temperature
        .unwrap_or_else(|| if run_count > 1 { 0.75 } else { 0.3 });
    let mut runs: Vec<(
        rlm::RlmAnalysisResult,
        Vec<rlm::TraceStep>,
        rlm::context_trace::ContextTraceSummary,
    )> = Vec::with_capacity(run_count);

    for _ in 0..run_count {
        let mut executor = rlm::RlmExecutor::new(content.clone(), provider.clone(), model.clone())
            .with_temperature(run_temperature)
            .with_verbose(args.verbose);
        let mut result = executor.analyze(&args.query).await?;
        if result.sub_queries.is_empty() {
            result.sub_queries.push(rlm::SubQuery {
                query: args.query.clone(),
                context_slice: if args.file.len() == 1 {
                    Some(args.file[0].to_string_lossy().to_string())
                } else {
                    None
                },
                response: result.answer.clone(),
                tokens_used: result.stats.output_tokens,
            });
        }
        runs.push((
            result,
            executor.trace_steps().to_vec(),
            executor.context_trace_summary(),
        ));
    }

    let result = runs
        .first()
        .map(|(r, _, _)| r.clone())
        .ok_or_else(|| anyhow::anyhow!("No RLM runs executed"))?;
    let source_path = if args.file.len() == 1 {
        Some(args.file[0].to_string_lossy().to_string())
    } else {
        None
    };

    let oracle_enabled = !args.no_oracle_verify;
    let mut oracle_json: serde_json::Value = serde_json::json!({
        "status": "disabled",
        "reason": "disabled via --no-oracle-verify",
    });
    let mut oracle_status = "[oracle: disabled —] verification disabled".to_string();

    if oracle_enabled {
        let validator =
            rlm::TraceValidator::new().with_consensus_threshold(args.consensus_threshold);
        let run_results: Vec<rlm::RlmAnalysisResult> =
            runs.iter().map(|(r, _, _)| r.clone()).collect();
        let trace_steps = runs.first().map(|(_, t, _)| t.clone()).unwrap_or_default();
        let oracle_result = if run_results.len() > 1 {
            validator.validate_with_consensus(
                &run_results,
                &content,
                source_path.as_deref(),
                None,
                Some(trace_steps),
            )
        } else {
            validator.validate(
                &run_results[0],
                &content,
                source_path.as_deref(),
                None,
                Some(trace_steps),
            )
        };

        let mut stats = rlm::oracle::BatchValidationStats::default();
        match &oracle_result {
            rlm::OracleResult::Golden(trace) => stats.golden.push(trace.clone()),
            rlm::OracleResult::Consensus { trace, .. } => stats.consensus.push(trace.clone()),
            rlm::OracleResult::Unverified { reason, trace } => {
                stats.unverified.push((trace.clone(), reason.clone()));
            }
            rlm::OracleResult::Failed { reason, trace, .. } => {
                stats.failed.push((trace.clone(), reason.clone()));
            }
        }

        let split = if let Some(ref out_dir) = args.oracle_out_dir {
            let out_dir_str = out_dir.to_str().ok_or_else(|| {
                anyhow::anyhow!(
                    "Oracle output directory path is not valid UTF-8: {}",
                    out_dir.display()
                )
            })?;
            Some(stats.write_jsonl_split(out_dir_str, &args.oracle_prefix)?)
        } else {
            None
        };

        let storage = rlm::OracleTraceStorage::from_env_or_vault().await;
        let persist = match storage.persist_result(&oracle_result).await {
            Ok(p) => Some(p),
            Err(e) => Some(rlm::OracleTracePersistResult {
                verdict: oracle_result.to_record().verdict,
                spooled_path: String::new(),
                uploaded: false,
                remote_key: None,
                remote_url: None,
                pending_count: 0,
                warning: Some(format!("Failed to persist oracle record: {}", e)),
            }),
        };

        oracle_status = oracle_status_line(&oracle_result);
        oracle_json = oracle_json_value(&oracle_result, split, persist);
    }

    if args.json {
        let trace_steps_json = runs
            .first()
            .map(|(_, t, _)| serde_json::to_value(t).unwrap_or_default())
            .unwrap_or_else(|| serde_json::json!([]));
        let context_trace_json = runs
            .first()
            .map(|(_, _, c)| serde_json::to_value(c).unwrap_or_default())
            .unwrap_or_else(|| serde_json::json!({}));
        let context_traces: Vec<serde_json::Value> = runs
            .iter()
            .map(|(_, _, c)| serde_json::to_value(c).unwrap_or_default())
            .collect();
        let output = serde_json::json!({
            "query": args.query,
            "provider": provider_name,
            "model": model,
            "content_type": format!("{:?}", content_type),
            "input_tokens": input_tokens,
            "answer": result.answer,
            "iterations": result.iterations,
            "sub_queries": result.sub_queries.len(),
            "stats": result.stats,
            "trace_steps": trace_steps_json,
            "context_trace": context_trace_json,
            "runs": run_count,
            "context_traces": context_traces,
            "oracle": oracle_json,
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("## RLM Analysis Result\n");
        println!("**Query:** {}\n", args.query);
        println!("**Answer:**\n{}\n", result.answer);
        println!("{}", oracle_status);
        println!("---");
        println!(
            "*Iterations: {} | Sub-queries: {} | Time: {}ms*",
            result.iterations,
            result.sub_queries.len(),
            result.stats.elapsed_ms
        );
        if let Some((_, trace_steps, trace_summary)) = runs.first() {
            println!(
                "\n*Trace steps: {} | Context budget used: {:.1}%*",
                trace_steps.len(),
                trace_summary.budget_used_percent
            );
        }

        if !result.sub_queries.is_empty() {
            println!("\n### Sub-queries made:");
            for (i, sq) in result.sub_queries.iter().enumerate() {
                println!("{}. {} -> {} tokens", i + 1, sq.query, sq.tokens_used);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load local .env for developer workflows (e.g. `cargo run` without exported vars).
    // Existing process environment still takes precedence over .env values.
    let _ = dotenvy::dotenv();

    // Install the rustls crypto provider before any TLS usage (reqwest, hyper-rustls, etc.)
    // Both aws-lc-rs and ring are in the dependency tree, so rustls cannot auto-detect.
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    let cli = Cli::parse();

    // Check if we're running TUI - if so, redirect logs to file instead of stderr
    // TUI is the default when no subcommand is given
    let is_tui = matches!(cli.command, Some(Command::Tui(_)) | None);

    // Initialize tracing
    if is_tui {
        // For TUI, log to file to avoid corrupting the display
        let log_dir = directories::ProjectDirs::from("com", "codetether", "codetether-agent")
            .map(|p| p.data_dir().to_path_buf())
            .unwrap_or_else(|| std::path::PathBuf::from("/tmp"));
        let _ = std::fs::create_dir_all(&log_dir);
        let log_file = std::fs::File::create(log_dir.join("tui.log")).ok();

        if let Some(file) = log_file {
            tracing_subscriber::registry()
                .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
                .with(
                    tracing_subscriber::fmt::layer()
                        .with_writer(std::sync::Mutex::new(file))
                        .with_ansi(false),
                )
                .init();
        }
        // If file creation fails, just don't log
    } else {
        tracing_subscriber::registry()
            .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
            .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
            .init();
    }

    let app_config = match config::Config::load().await {
        Ok(cfg) => cfg,
        Err(err) => {
            tracing::warn!(
                error = %err,
                "Failed to load config for crash reporter; using defaults"
            );
            config::Config::default()
        }
    };
    let allow_crash_prompt =
        is_tui && std::io::stdin().is_terminal() && std::io::stdout().is_terminal();
    let app_config = crash::maybe_prompt_for_consent(&app_config, allow_crash_prompt).await;
    crash::initialize(&app_config).await;

    // Initialize HashiCorp Vault connection for secrets
    if let Ok(secrets_manager) = secrets::SecretsManager::from_env().await {
        if secrets_manager.is_connected() {
            tracing::info!("Connected to HashiCorp Vault for secrets management");
        }
        // Store in global
        let _ = secrets::init_from_manager(secrets_manager);
    } else {
        tracing::warn!(
            "HashiCorp Vault not configured - Vault provider API keys will be unavailable"
        );
        tracing::warn!("Set VAULT_ADDR and VAULT_TOKEN environment variables to connect");
    }

    match cli.command {
        Some(Command::Tui(args)) => {
            let project = args.project.or(cli.project);
            tui::run(project).await
        }
        Some(Command::Serve(args)) => server::serve(args).await,
        Some(Command::Run(args)) => cli::run::execute(args).await,
        Some(Command::Models(args)) => {
            let registry = provider::ProviderRegistry::from_vault().await?;
            let mut all_models: Vec<provider::ModelInfo> = Vec::new();

            for provider_name in registry.list() {
                if let Some(filter) = &args.provider {
                    if provider_name != filter.as_str() {
                        continue;
                    }
                }
                if let Some(p) = registry.get(provider_name) {
                    match p.list_models().await {
                        Ok(models) => all_models.extend(models),
                        Err(e) => eprintln!(
                            "Warning: failed to list models for {}: {}",
                            provider_name, e
                        ),
                    }
                }
            }

            all_models.sort_by(|a, b| a.provider.cmp(&b.provider).then(a.id.cmp(&b.id)));

            if args.json {
                println!("{}", serde_json::to_string_pretty(&all_models)?);
            } else {
                let mut current_provider = String::new();
                for m in &all_models {
                    if m.provider != current_provider {
                        if !current_provider.is_empty() {
                            println!();
                        }
                        println!("\x1b[1;36m{}\x1b[0m", m.provider);
                        current_provider = m.provider.clone();
                    }
                    let ctx = m.context_window / 1000;
                    let out = m
                        .max_output_tokens
                        .map(|t| format!("{}k", t / 1000))
                        .unwrap_or_else(|| "-".to_string());
                    println!("  {:<45} {:>5}k ctx  {:>5} out", m.id, ctx, out);
                }
                println!(
                    "\n\x1b[2m{} models from {} providers\x1b[0m",
                    all_models.len(),
                    {
                        let mut providers: Vec<&str> =
                            all_models.iter().map(|m| m.provider.as_str()).collect();
                        providers.dedup();
                        providers.len()
                    }
                );
            }
            Ok(())
        }
        Some(Command::Auth(args)) => cli::auth::execute(args).await,
        Some(Command::Worker(mut args)) => {
            // Auto-load saved credentials if no token provided
            if args.token.is_none() {
                if let Some(creds) = cli::auth::load_saved_credentials() {
                    let target = args.server.trim_end_matches('/');
                    let saved = creds.server.trim_end_matches('/');
                    if saved == target {
                        tracing::info!(email = %creds.email, server = %saved, "Using saved credentials from `codetether auth login`");
                        args.token = Some(creds.access_token);
                    } else {
                        tracing::warn!(saved_server = %saved, target_server = %target, "Ignoring saved credentials for different server");
                    }
                }
            }

            if args.no_http_server {
                a2a::worker::run(args).await
            } else {
                // Create shared state for worker and HTTP server communication
                let worker_state = worker_server::WorkerServerState::new();

                let http_args = cli::WorkerServerArgs {
                    hostname: args.hostname.clone(),
                    port: args.port,
                };
                // Clone the state so both worker and HTTP server share the same underlying state
                let shared_state = worker_state.clone();
                let http_server_handle = tokio::spawn(async move {
                    // Use the same state as the worker - it will receive updates via set_connected/set_worker_id
                    if let Err(e) =
                        worker_server::start_worker_server_with_state(http_args, shared_state).await
                    {
                        tracing::error!("Worker HTTP server error: {}", e);
                    }
                });

                // Run the SSE worker with the shared state
                let result = a2a::worker::run_with_state(args, worker_state).await;
                http_server_handle.abort();
                result
            }
        }
        Some(Command::Spawn(args)) => a2a::spawn::run(args).await,
        Some(Command::Config(args)) => cli::config::execute(args).await,
        Some(Command::Swarm(args)) => {
            let executor = SwarmExecutor::new(swarm::SwarmConfig {
                max_subagents: args.max_subagents,
                max_steps_per_subagent: args.max_steps,
                subagent_timeout_secs: args.timeout,
                model: args.model.clone().or_else(|| Some("zai/glm-5".to_string())),
                execution_mode: ExecutionMode::from_cli_value(&args.execution_mode),
                k8s_pod_budget: args.k8s_pod_budget,
                k8s_subagent_image: args.k8s_image.clone(),
                ..Default::default()
            });

            let strategy = match args.strategy.as_str() {
                "auto" => DecompositionStrategy::Automatic,
                "domain" => DecompositionStrategy::ByDomain,
                "data" => DecompositionStrategy::ByData,
                "stage" => DecompositionStrategy::ByStage,
                "none" => DecompositionStrategy::None,
                _ => DecompositionStrategy::Automatic,
            };

            let result = executor.execute(&args.task, strategy).await?;

            if args.json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                println!("=== Swarm Execution Results ===");
                println!(
                    "Status: {}",
                    if result.success { "SUCCESS" } else { "FAILED" }
                );
                println!("Subtasks: {}", result.subtask_results.len());
                println!("Speedup: {:.1}x", result.stats.speedup_factor);
                println!("Critical Path: {} steps", result.stats.critical_path_length);
                println!("\n{}", result.result);
            }
            Ok(())
        }
        Some(Command::SwarmSubagent(args)) => swarm::remote_subtask::run_swarm_subagent(args).await,
        Some(Command::Rlm(args)) => run_rlm_command(args).await,
        Some(Command::Oracle(args)) => cli::oracle::execute(args).await,
        Some(Command::Ralph(args)) => {
            use provider::ProviderRegistry;

            match args.action.as_str() {
                "create-prd" => {
                    let project = args.project_name.as_deref().unwrap_or("my-project");
                    let feature = args.feature.as_deref().unwrap_or("new-feature");
                    let prd = ralph::create_prd_template(project, feature);

                    if args.json {
                        println!("{}", serde_json::to_string_pretty(&prd)?);
                    } else {
                        println!("{}", serde_json::to_string_pretty(&prd)?);
                        eprintln!("\nSave this to prd.json and run: codetether ralph run");
                    }
                    Ok(())
                }
                "status" => {
                    let prd = ralph::Prd::load(&args.prd).await?;
                    let stories: Vec<String> = prd
                        .user_stories
                        .iter()
                        .map(|s| {
                            let check = if s.passes { "[x]" } else { "[ ]" };
                            format!("- {} {}: {}", check, s.id, s.title)
                        })
                        .collect();

                    println!("# Ralph Status\n");
                    println!("**Project:** {}", prd.project);
                    println!("**Feature:** {}", prd.feature);
                    println!("**Branch:** {}", prd.branch_name);
                    println!(
                        "**Progress:** {}/{} stories\n",
                        prd.passed_count(),
                        prd.user_stories.len()
                    );
                    println!("## Stories\n{}", stories.join("\n"));
                    Ok(())
                }
                "run" => {
                    tracing::info!("Starting Ralph loop with PRD: {}", args.prd.display());

                    let registry = ProviderRegistry::from_vault().await?;
                    let provider = registry
                        .get("zai")
                        .or_else(|| registry.get("moonshotai"))
                        .or_else(|| registry.get("openai"))
                        .ok_or_else(|| anyhow::anyhow!("No provider available in Vault"))?;

                    let model = args.model.unwrap_or_else(|| "glm-5".to_string());
                    let config = ralph::RalphConfig {
                        prd_path: args.prd.to_string_lossy().to_string(),
                        max_iterations: args.max_iterations,
                        ..Default::default()
                    };

                    let mut loop_runner =
                        ralph::RalphLoop::new(args.prd.clone(), provider, model, config).await?;

                    let result = loop_runner.run().await?;

                    if args.json {
                        println!("{}", serde_json::to_string_pretty(&result)?);
                    } else {
                        println!("{}", loop_runner.status_markdown());
                    }
                    Ok(())
                }
                _ => {
                    anyhow::bail!(
                        "Unknown action: {}. Use run, status, or create-prd",
                        args.action
                    );
                }
            }
        }
        Some(Command::Mcp(args)) => {
            match args.action.as_str() {
                "serve" => {
                    tracing::info!("Starting MCP server over stdio...");

                    // Create agent bus + S3 sink so tool calls sync to MinIO
                    let bus = crate::bus::AgentBus::new().into_arc();
                    crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());

                    // Try to load provider from Vault so tools like Ralph can execute autonomously
                    let registry = match crate::provider::ProviderRegistry::from_vault().await {
                        Ok(provider_registry) => {
                            let fallbacks = [
                                "zai",
                                "openai",
                                "github-copilot",
                                "anthropic",
                                "openrouter",
                                "novita",
                                "moonshotai",
                                "google",
                            ];
                            let provider_and_model = fallbacks
                                .iter()
                                .find_map(|name| provider_registry.get(name).map(|p| (p, name)));
                            match provider_and_model {
                                Some((provider, name)) => {
                                    let model = match *name {
                                        "zai" => "glm-5".to_string(),
                                        "anthropic" => "claude-sonnet-4-20250514".to_string(),
                                        "openai" => "gpt-4.1".to_string(),
                                        _ => "default".to_string(),
                                    };
                                    tracing::info!(provider = %name, model = %model, "MCP server loaded provider from Vault");
                                    std::sync::Arc::new(tool::ToolRegistry::with_provider(
                                        provider, model,
                                    ))
                                }
                                None => {
                                    tracing::warn!(
                                        "Vault connected but no known provider found, Ralph/Go will be limited"
                                    );
                                    std::sync::Arc::new(tool::ToolRegistry::with_defaults())
                                }
                            }
                        }
                        Err(e) => {
                            tracing::warn!(error = %e, "Failed to load providers from Vault, Ralph/Go will be limited");
                            std::sync::Arc::new(tool::ToolRegistry::with_defaults())
                        }
                    };
                    let server = mcp::McpServer::new_stdio()
                        .with_tool_registry(registry)
                        .with_agent_bus(bus);
                    let server = if let Some(bus_url) = &args.bus_url {
                        tracing::info!(bus_url = %bus_url, "Connecting MCP server to agent bus");
                        server.with_bus(bus_url.clone()).await
                    } else {
                        server
                    };
                    server.run().await?;
                    Ok(())
                }
                "connect" => {
                    let command = args
                        .command
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("--command required for connect action"))?;

                    // Parse command into parts
                    let parts: Vec<&str> = command.split_whitespace().collect();
                    if parts.is_empty() {
                        anyhow::bail!("Empty command");
                    }

                    let cmd = parts[0];
                    let cmd_args: Vec<&str> = parts[1..].to_vec();

                    tracing::info!("Connecting to MCP server: {}", command);
                    let client = mcp::McpClient::connect_subprocess(cmd, &cmd_args).await?;

                    // List available tools
                    let tools = client.tools().await;

                    if args.json {
                        println!("{}", serde_json::to_string_pretty(&tools)?);
                    } else {
                        println!("# Connected to MCP Server\n");
                        println!("## Available Tools ({})\n", tools.len());
                        for tool in &tools {
                            println!(
                                "- **{}**: {}",
                                tool.name,
                                tool.description.as_deref().unwrap_or("")
                            );
                        }
                    }

                    // Keep connection alive briefly to show it works
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    client.close().await?;

                    Ok(())
                }
                "list-tools" => {
                    if let Some(command) = args.command.as_ref() {
                        // Connect to external MCP server and list its tools
                        let parts: Vec<&str> = command.split_whitespace().collect();
                        if parts.is_empty() {
                            anyhow::bail!("Empty command");
                        }
                        let cmd = parts[0];
                        let cmd_args: Vec<&str> = parts[1..].to_vec();
                        let client = mcp::McpClient::connect_subprocess(cmd, &cmd_args).await?;
                        let tools = client.tools().await;
                        if args.json {
                            println!("{}", serde_json::to_string_pretty(&tools)?);
                        } else {
                            println!("# Available MCP Tools\n");
                            for tool in &tools {
                                println!(
                                    "- **{}**: {}",
                                    tool.name,
                                    tool.description.as_deref().unwrap_or("")
                                );
                            }
                        }
                        client.close().await?;
                    } else {
                        // List built-in tools from the MCP server registry
                        let registry = std::sync::Arc::new(tool::ToolRegistry::with_defaults());
                        let server = mcp::McpServer::new_local().with_tool_registry(registry);
                        server.setup_tools_public().await;
                        let tools = server.get_all_tool_metadata().await;
                        if args.json {
                            println!("{}", serde_json::to_string_pretty(&tools)?);
                        } else {
                            println!("# Available MCP Tools\n");
                            for tool in &tools {
                                println!(
                                    "- **{}**: {}",
                                    tool.name,
                                    tool.description.as_deref().unwrap_or("")
                                );
                            }
                        }
                    }
                    Ok(())
                }
                "call" => {
                    let tool = args
                        .tool
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("--tool required for call action"))?;
                    let arguments: serde_json::Value = args
                        .arguments
                        .as_ref()
                        .map(|s| serde_json::from_str(s))
                        .transpose()?
                        .unwrap_or(serde_json::json!({}));

                    if let Some(command) = args.command.as_ref() {
                        // Call tool on external MCP server
                        let parts: Vec<&str> = command.split_whitespace().collect();
                        if parts.is_empty() {
                            anyhow::bail!("Empty command");
                        }
                        let cmd = parts[0];
                        let cmd_args: Vec<&str> = parts[1..].to_vec();
                        let client = mcp::McpClient::connect_subprocess(cmd, &cmd_args).await?;
                        let result = client.call_tool(tool, arguments).await?;
                        client.close().await?;

                        let output: String = result
                            .content
                            .iter()
                            .map(|c| match c {
                                mcp::ToolContent::Text { text } => text.clone(),
                                mcp::ToolContent::Image { data, mime_type } => {
                                    format!("[image: {} ({} bytes)]", mime_type, data.len())
                                }
                                mcp::ToolContent::Resource { resource } => {
                                    serde_json::to_string(resource).unwrap_or_default()
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("\n");

                        if result.is_error {
                            eprintln!("Error: {}", output);
                        } else {
                            println!("{}", output);
                        }
                    } else {
                        // Call built-in tool directly via MCP server
                        let registry = std::sync::Arc::new(tool::ToolRegistry::with_defaults());
                        let server = mcp::McpServer::new_local().with_tool_registry(registry);
                        server.setup_tools_public().await;
                        let result = server.call_tool_direct(tool, arguments).await?;

                        let output: String = result
                            .content
                            .iter()
                            .map(|c| match c {
                                mcp::ToolContent::Text { text } => text.clone(),
                                mcp::ToolContent::Image { data, mime_type } => {
                                    format!("[image: {} ({} bytes)]", mime_type, data.len())
                                }
                                mcp::ToolContent::Resource { resource } => {
                                    serde_json::to_string(resource).unwrap_or_default()
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("\n");

                        if result.is_error {
                            eprintln!("Error: {}", output);
                        } else {
                            println!("{}", output);
                        }
                    }
                    Ok(())
                }
                _ => {
                    anyhow::bail!(
                        "Unknown action: {}. Use serve, connect, list-tools, or call",
                        args.action
                    );
                }
            }
        }
        Some(Command::Stats(args)) => {
            // Show telemetry statistics - use persistent data
            let persistent = get_persistent_stats();
            let token_snapshot = TOKEN_USAGE.global_snapshot();

            if args.json {
                let mut output = serde_json::json!({
                    "tool_executions": persistent.stats,
                    "token_usage": {
                        "total_input": persistent.stats.total_input_tokens,
                        "total_output": persistent.stats.total_output_tokens,
                        "requests": persistent.stats.total_requests,
                    }
                });

                if args.tools || args.all {
                    let recent: Vec<_> = persistent.recent(args.limit).into_iter().collect();
                    output["recent_executions"] = serde_json::json!(recent);
                }

                if args.files || args.all {
                    let changes = persistent.all_file_changes();
                    let recent_changes: Vec<_> = changes.iter().rev().take(args.limit).collect();
                    output["recent_file_changes"] = serde_json::json!(recent_changes);
                }

                if let Some(ref tool_name) = args.tool {
                    let tool_execs = persistent.by_tool(tool_name);
                    output["tool_filter"] = serde_json::json!(tool_execs);
                }

                if let Some(ref file_path) = args.file {
                    let file_execs = persistent.by_file(file_path);
                    output["file_filter"] = serde_json::json!(file_execs);
                }

                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!("# CodeTether Telemetry\n");

                // Summary
                println!("## Summary\n");
                println!("**Tool Executions:** {}", persistent.summary());
                println!("**Token Usage:** {}", token_snapshot.summary());

                // Tool breakdown
                if !persistent.stats.executions_by_tool.is_empty() {
                    println!("\n## Tool Breakdown\n");
                    let mut tools: Vec<_> = persistent.stats.executions_by_tool.iter().collect();
                    tools.sort_by(|a, b| b.1.cmp(a.1));
                    for (tool, count) in tools.iter().take(10) {
                        println!("- **{}**: {} executions", tool, count);
                    }
                }

                // Files modified
                if !persistent.stats.files_modified.is_empty() {
                    println!("\n## Files Modified\n");
                    let mut files: Vec<_> = persistent.stats.files_modified.iter().collect();
                    files.sort_by(|a, b| b.1.cmp(a.1));
                    for (path, count) in files.iter().take(10) {
                        println!("- **{}**: {} changes", path, count);
                    }
                }

                // Recent executions
                if args.tools || args.all {
                    let recent = persistent.recent(args.limit);
                    if !recent.is_empty() {
                        println!("\n## Recent Tool Executions\n");
                        for exec in recent {
                            let status = if exec.success { "✓" } else { "✗" };
                            let files_str = if exec.file_changes.is_empty() {
                                String::new()
                            } else {
                                format!(
                                    " → {}",
                                    exec.file_changes
                                        .iter()
                                        .map(|f: &telemetry::FileChange| f.summary())
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                )
                            };
                            println!(
                                "- {} **{}** ({}ms){}",
                                status, exec.tool_name, exec.duration_ms, files_str
                            );
                        }
                    }
                }

                // Recent file changes
                if args.files || args.all {
                    let changes = persistent.all_file_changes();
                    if !changes.is_empty() {
                        println!("\n## Recent File Changes\n");
                        for (exec_id, change) in changes.iter().rev().take(args.limit) {
                            println!("- [#{}] {}", exec_id, change.summary());
                        }
                    }
                }

                // Token usage by model
                if args.tokens || args.all {
                    let model_snapshots = TOKEN_USAGE.model_snapshots();
                    if !model_snapshots.is_empty() {
                        println!("\n## Token Usage by Model\n");
                        for snapshot in &model_snapshots {
                            println!("- **{}**: {}", snapshot.name, snapshot.summary());
                        }
                    }
                }

                // Filtered results
                if let Some(ref tool_name) = args.tool {
                    let tool_execs = persistent.by_tool(tool_name);
                    if !tool_execs.is_empty() {
                        println!("\n## Executions of '{}'\n", tool_name);
                        for exec in tool_execs.iter().take(args.limit) {
                            let status = if exec.success { "✓" } else { "✗" };
                            println!("- {} ({}ms)", status, exec.duration_ms);
                        }
                    }
                }

                if let Some(ref file_path) = args.file {
                    let file_execs = persistent.by_file(file_path);
                    if !file_execs.is_empty() {
                        println!("\n## Changes to '{}'\n", file_path);
                        for exec in file_execs.iter().take(args.limit) {
                            println!("- **{}** ({}ms)", exec.tool_name, exec.duration_ms);
                            for change in &exec.file_changes {
                                if change.path == *file_path {
                                    if let Some((start, end)) = change.line_range {
                                        println!("  Lines {}-{}", start, end);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        }
        Some(Command::Benchmark(args)) => {
            if args.models.is_empty() {
                anyhow::bail!("At least one model is required (--models provider:model)");
            }

            let config = benchmark::BenchmarkConfig {
                prd_dir: args.prd_dir,
                models: args.models,
                tier: args.tier,
                parallel: args.parallel,
                max_iterations: args.max_iterations,
                story_timeout_secs: args.story_timeout,
                output: args.output,
                cost_ceiling_usd: Some(args.cost_ceiling),
                submit_api_url: args.submit_url,
                submit_api_key: args.submit_key,
            };

            let runner = benchmark::BenchmarkRunner::new(config);
            let result = runner.run().await?;

            if args.json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                println!("=== Benchmark Results ===\n");
                println!("Date: {}", result.run_date);
                println!("Agent: {} v{}\n", result.agent, result.agent_version);

                for mr in &result.model_results {
                    println!("--- {} ---", mr.model);
                    println!(
                        "  Pass Rate: {:.1}% ({}/{} stories)",
                        mr.aggregate.overall_pass_rate * 100.0,
                        mr.aggregate.total_stories_passed,
                        mr.aggregate.total_stories
                    );
                    println!(
                        "  Speed: {:.1} stories/hour ({:.1}s avg)",
                        mr.aggregate.stories_per_hour, mr.aggregate.avg_seconds_per_story
                    );
                    println!(
                        "  Cost: ${:.4}/story (${:.2} total)",
                        mr.aggregate.avg_cost_per_story, mr.aggregate.total_cost_usd
                    );
                    println!(
                        "  PRDs: {}/{} fully passed\n",
                        mr.aggregate.prds_fully_passed, mr.aggregate.prds_attempted
                    );
                }

                if !result.summary.rankings.is_empty() {
                    println!("=== Rankings ===");
                    println!("Best Pass Rate: {}", result.summary.best_pass_rate_model);
                    println!("Fastest: {}", result.summary.fastest_model);
                    println!("Cheapest: {}", result.summary.cheapest_model);
                    println!("Best Overall: {}", result.summary.best_overall_model);
                }
            }
            Ok(())
        }
        Some(Command::Cleanup(args)) => {
            // Clean up orphaned worktrees and branches from Ralph runs
            use worktree::WorktreeManager;

            let cwd = std::env::current_dir()?;
            let mgr = WorktreeManager::new(&cwd);

            // List what exists
            let worktrees = mgr.list().await;

            if args.dry_run {
                if args.json {
                    let output = serde_json::json!({
                        "dry_run": true,
                        "worktrees_found": worktrees.len(),
                        "worktrees": worktrees.iter().map(|wt| {
                            serde_json::json!({
                                "id": wt.name,
                                "branch": wt.branch,
                                "path": wt.path.display().to_string(),
                            })
                        }).collect::<Vec<_>>(),
                    });
                    println!("{}", serde_json::to_string_pretty(&output)?);
                } else {
                    println!("# Cleanup Preview (Dry Run)\n");
                    if worktrees.is_empty() {
                        println!("No orphaned worktrees or branches found.");
                    } else {
                        println!("Found {} worktree(s) to clean:\n", worktrees.len());
                        for wt in &worktrees {
                            println!("- **{}** → {}", wt.branch, wt.path.display());
                        }
                        println!("\nRun without --dry-run to delete.");
                    }
                }
            } else {
                let count = mgr.cleanup_all().await?;

                if args.json {
                    println!(
                        "{}",
                        serde_json::json!({
                            "cleaned": count,
                            "success": true,
                        })
                    );
                } else if count > 0 {
                    println!("Cleaned up {} orphaned worktree(s)/branch(es).", count);
                } else {
                    println!("No orphaned worktrees or branches found.");
                }
            }
            Ok(())
        }
        Some(Command::Moltbook(args)) => {
            use cli::MoltbookCommand;

            match args.command {
                MoltbookCommand::Register(reg) => {
                    println!("🦞 Registering on Moltbook as '{}'...\n", reg.name);
                    let result =
                        moltbook::MoltbookClient::register(&reg.name, reg.description.as_deref())
                            .await?;

                    println!("✅ Registered successfully!\n");
                    println!("   Agent:             {}", reg.name);
                    println!("   API Key:           {}", result.agent.api_key);
                    println!("   Claim URL:         {}", result.agent.claim_url);
                    println!("   Verification Code: {}", result.agent.verification_code);
                    println!("\n🔗 Send the claim URL to your human to verify ownership.");
                    println!("🔐 API key has been saved to Vault (codetether/moltbook).");
                    Ok(())
                }
                MoltbookCommand::Status => {
                    let client = moltbook::MoltbookClient::from_vault_or_env().await?;
                    let status = client.claim_status().await?;
                    println!("Moltbook claim status: {}", status.status);
                    Ok(())
                }
                MoltbookCommand::Profile => {
                    let client = moltbook::MoltbookClient::from_vault_or_env().await?;
                    let profile = client.me().await?;
                    println!("# Moltbook Profile\n");
                    println!("**Name:** {}", profile.name);
                    if let Some(desc) = &profile.description {
                        println!("**Description:** {}", desc);
                    }
                    if let Some(k) = profile.karma {
                        println!("**Karma:** {}", k);
                    }
                    if let Some(f) = profile.follower_count {
                        println!("**Followers:** {}", f);
                    }
                    if let Some(claimed) = profile.is_claimed {
                        println!("**Claimed:** {}", if claimed { "Yes" } else { "No" });
                    }
                    println!("\n🔗 https://www.moltbook.com/u/{}", profile.name);
                    Ok(())
                }
                MoltbookCommand::UpdateProfile(upd) => {
                    let client = moltbook::MoltbookClient::from_vault_or_env().await?;
                    client.update_profile(upd.description.as_deref()).await?;
                    println!("✅ Profile updated (CodeTether branding included).");
                    Ok(())
                }
                MoltbookCommand::Post(post) => {
                    let client = moltbook::MoltbookClient::from_vault_or_env().await?;
                    let resp = client
                        .create_post(&post.submolt, &post.title, &post.content)
                        .await?;
                    println!("✅ Posted to m/{}: {}", post.submolt, post.title);
                    tracing::debug!("Post response: {}", resp);
                    Ok(())
                }
                MoltbookCommand::Intro => {
                    let client = moltbook::MoltbookClient::from_vault_or_env().await?;
                    let profile = client.me().await?;
                    let (title, content) = moltbook::intro_post(&profile.name);
                    let resp = client
                        .create_post("introductions", &title, &content)
                        .await?;
                    println!("✅ CodeTether intro posted to m/introductions!");
                    println!("   Title: {}", title);
                    tracing::debug!("Post response: {}", resp);
                    Ok(())
                }
                MoltbookCommand::Heartbeat => {
                    let client = moltbook::MoltbookClient::from_vault_or_env().await?;
                    let summary = client.heartbeat().await?;
                    println!("{}", summary);
                    Ok(())
                }
                MoltbookCommand::Comment(c) => {
                    let client = moltbook::MoltbookClient::from_vault_or_env().await?;
                    let resp = client.comment(&c.post_id, &c.content).await?;
                    println!("✅ Commented on post {}", c.post_id);
                    tracing::debug!("Comment response: {}", resp);
                    Ok(())
                }
                MoltbookCommand::Search(s) => {
                    let client = moltbook::MoltbookClient::from_vault_or_env().await?;
                    let results = client.search(&s.query, s.limit).await?;
                    println!("{}", serde_json::to_string_pretty(&results)?);
                    Ok(())
                }
            }
        }

        // OKR commands
        Some(Command::Okr(args)) => {
            use crate::okr::{KeyResult, Okr, OkrRepository, OkrStatus};
            use uuid::Uuid;

            let repo = OkrRepository::from_config().await?;

            match args.action.as_str() {
                "list" => {
                    let okrs = if let Some(status) = &args.status {
                        let status = match status.as_str() {
                            "draft" => OkrStatus::Draft,
                            "active" => OkrStatus::Active,
                            "completed" => OkrStatus::Completed,
                            "cancelled" => OkrStatus::Cancelled,
                            "on_hold" => OkrStatus::OnHold,
                            _ => anyhow::bail!("Invalid status: {}", status),
                        };
                        repo.query_okrs_by_status(status).await?
                    } else if let Some(owner) = &args.owner {
                        repo.query_okrs_by_owner(owner).await?
                    } else {
                        repo.list_okrs().await?
                    };

                    if args.json {
                        println!("{}", serde_json::to_string_pretty(&okrs)?);
                    } else {
                        println!("\n=== OKRs ===\n");
                        for okr in &okrs {
                            let progress = okr.progress() * 100.0;
                            println!("[{}] {} - {:.1}% complete", okr.id, okr.title, progress);
                            println!("  Status: {:?}", okr.status);
                            if let Some(owner) = &okr.owner {
                                println!("  Owner: {}", owner);
                            }
                            for kr in &okr.key_results {
                                let kr_progress = kr.progress() * 100.0;
                                println!(
                                    "  - {}: {:.1}/{} {} ({:.1}%)",
                                    kr.title,
                                    kr.current_value,
                                    kr.target_value,
                                    kr.unit,
                                    kr_progress
                                );
                                if args.evidence && !kr.outcomes.is_empty() {
                                    for outcome in &kr.outcomes {
                                        println!("    Evidence: {}", outcome.description);
                                        for ev in &outcome.evidence {
                                            println!("      - {}", ev);
                                        }
                                    }
                                }
                            }
                            println!();
                        }
                    }
                    Ok(())
                }

                "status" => {
                    let id = args
                        .id
                        .ok_or_else(|| anyhow::anyhow!("--id required for status"))?;
                    let uuid = Uuid::parse_str(&id)?;

                    let okr = repo
                        .get_okr(uuid)
                        .await?
                        .ok_or_else(|| anyhow::anyhow!("OKR not found"))?;

                    if args.json {
                        println!("{}", serde_json::to_string_pretty(&okr)?);
                    } else {
                        println!("\n=== OKR: {} ===\n", okr.title);
                        println!("Description: {}", okr.description);
                        println!("Status: {:?}", okr.status);
                        println!("Progress: {:.1}%\n", okr.progress() * 100.0);

                        println!("Key Results:");
                        for kr in &okr.key_results {
                            let kr_progress = kr.progress() * 100.0;
                            println!("\n  [{}] {}", kr.id, kr.title);
                            println!(
                                "  Progress: {:.1}/{} {} ({:.1}%)",
                                kr.current_value, kr.target_value, kr.unit, kr_progress
                            );
                            println!("  Status: {:?}", kr.status);

                            if !kr.outcomes.is_empty() {
                                println!("  Outcomes:");
                                for outcome in &kr.outcomes {
                                    println!(
                                        "    - {} ({:?})",
                                        outcome.description, outcome.outcome_type
                                    );
                                    if let Some(value) = outcome.value {
                                        println!("      Value: {}", value);
                                    }
                                    for ev in &outcome.evidence {
                                        println!("      Evidence: {}", ev);
                                    }
                                }
                            }
                        }

                        // Show runs
                        let runs = repo.query_runs_by_okr(uuid).await?;
                        if !runs.is_empty() {
                            println!("\nRuns:");
                            for run in &runs {
                                println!("  [{}] {} - {:?}", run.id, run.name, run.status);
                                if let Some(corr) = &run.correlation_id {
                                    println!("    Correlation: {}", corr);
                                }
                            }
                        }
                    }
                    Ok(())
                }

                "create" => {
                    let title = args
                        .title
                        .ok_or_else(|| anyhow::anyhow!("--title required"))?;
                    let description = args.description.unwrap_or_default();

                    let mut okr = Okr::new(title, description);

                    // Add key result if target specified
                    if let Some(target) = args.target {
                        let kr = KeyResult::new(okr.id, "Key Result 1", target, args.unit);
                        okr.add_key_result(kr);
                    }

                    let created = repo.create_okr(okr).await?;
                    println!("Created OKR: {} ({})", created.title, created.id);
                    Ok(())
                }

                "runs" => {
                    let okrs = repo.list_okrs().await?;
                    let runs = repo.list_runs().await?;

                    if args.json {
                        #[derive(serde::Serialize)]
                        struct RunsOutput {
                            okrs: Vec<crate::okr::Okr>,
                            runs: Vec<crate::okr::OkrRun>,
                        }
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&RunsOutput { okrs, runs })?
                        );
                    } else {
                        println!("\n=== OKR Runs ===\n");
                        for run in &runs {
                            let okr_title = okrs
                                .iter()
                                .find(|o| o.id == run.okr_id)
                                .map(|o| o.title.as_str())
                                .unwrap_or("Unknown");

                            println!("[{}] {} -> {}", run.id, okr_title, run.name);
                            println!("  Status: {:?}", run.status);
                            if let Some(corr) = &run.correlation_id {
                                println!("  Correlation: {}", corr);
                            }
                            if let Some(session) = &run.session_id {
                                println!("  Session: {}", session);
                            }
                            println!();
                        }
                    }
                    Ok(())
                }

                "export" => {
                    let okrs = repo.list_okrs().await?;
                    let runs = repo.list_runs().await?;

                    #[derive(serde::Serialize)]
                    struct ExportData {
                        okrs: Vec<crate::okr::Okr>,
                        runs: Vec<crate::okr::OkrRun>,
                        exported_at: chrono::DateTime<chrono::Utc>,
                    }

                    let export = ExportData {
                        okrs,
                        runs,
                        exported_at: chrono::Utc::now(),
                    };

                    if args.json {
                        println!("{}", serde_json::to_string_pretty(&export)?);
                    } else {
                        println!(
                            "Exported {} OKRs and {} runs",
                            export.okrs.len(),
                            export.runs.len()
                        );
                    }
                    Ok(())
                }

                "stats" => {
                    let stats = repo.stats().await?;

                    if args.json {
                        println!("{}", serde_json::to_string_pretty(&stats)?);
                    } else {
                        println!("\n=== OKR Statistics ===\n");
                        println!("Total OKRs: {}", stats.total_okrs);
                        println!("Total Runs: {}", stats.total_runs);

                        println!("\nOKR Status:");
                        for (status, count) in &stats.okr_status_counts {
                            println!("  {:?}: {}", status, count);
                        }

                        println!("\nRun Status:");
                        for (status, count) in &stats.run_status_counts {
                            println!("  {:?}: {}", status, count);
                        }
                    }
                    Ok(())
                }

                "report" => {
                    let id = args
                        .id
                        .ok_or_else(|| anyhow::anyhow!("--id required for report"))?;
                    let uuid = Uuid::parse_str(&id)?;

                    // Try to find as OKR first, then as run
                    let okr = repo.get_okr(uuid).await?;
                    let run = repo.get_run(uuid).await?;

                    if let Some(okr) = okr {
                        // Report on an OKR
                        if args.json {
                            #[derive(serde::Serialize)]
                            struct OkrReport {
                                okr: crate::okr::Okr,
                                runs: Vec<crate::okr::OkrRun>,
                                total_progress: f64,
                            }
                            let runs = repo.query_runs_by_okr(uuid).await?;
                            let report = OkrReport {
                                okr: okr.clone(),
                                runs,
                                total_progress: okr.progress() * 100.0,
                            };
                            println!("{}", serde_json::to_string_pretty(&report)?);
                        } else {
                            println!("\n=== OKR Report: {} ===\n", okr.title);
                            println!("Description: {}", okr.description);
                            println!("Status: {:?}", okr.status);
                            println!("Overall Progress: {:.1}%\n", okr.progress() * 100.0);

                            println!("Key Results:");
                            for kr in &okr.key_results {
                                let kr_progress = kr.progress() * 100.0;
                                println!("\n  [{}] {}", kr.id, kr.title);
                                println!(
                                    "  Progress: {:.1}/{} {} ({:.1}%)",
                                    kr.current_value, kr.target_value, kr.unit, kr_progress
                                );
                                println!("  Status: {:?}", kr.status);

                                if !kr.outcomes.is_empty() {
                                    println!("  Outcomes:");
                                    for outcome in &kr.outcomes {
                                        println!(
                                            "    - {} ({:?})",
                                            outcome.description, outcome.outcome_type
                                        );
                                        if let Some(value) = outcome.value {
                                            println!("      Value: {}", value);
                                        }
                                        if args.evidence {
                                            for ev in &outcome.evidence {
                                                println!("      Evidence: {}", ev);
                                            }
                                        }
                                    }
                                } else if args.evidence {
                                    println!("  (No outcomes recorded)");
                                }
                            }

                            // Show runs for this OKR
                            let runs = repo.query_runs_by_okr(uuid).await?;
                            if !runs.is_empty() {
                                println!("\nExecution Runs:");
                                for run in &runs {
                                    println!("\n  [{}] {}", run.id, run.name);
                                    println!("  Status: {:?}", run.status);
                                    if let Some(corr) = &run.correlation_id {
                                        println!("  Correlation ID: {}", corr);
                                    }
                                    if let Some(session) = &run.session_id {
                                        println!("  Session: {}", session);
                                    }
                                    if let Some(checkpoint) = &run.relay_checkpoint_id {
                                        println!("  Relay Checkpoint: {}", checkpoint);
                                    }
                                    if !run.kr_progress.is_empty() {
                                        println!("  KR Progress:");
                                        for (kr_id, progress) in &run.kr_progress {
                                            println!("    {}: {:.1}%", kr_id, progress * 100.0);
                                        }
                                    }
                                }
                            }
                        }
                        Ok(())
                    } else if let Some(run) = run {
                        // Report on a specific run
                        if args.json {
                            // Get the associated OKR
                            let okr = repo.get_okr(run.okr_id).await?;
                            #[derive(serde::Serialize)]
                            struct RunReport {
                                run: crate::okr::OkrRun,
                                okr: Option<crate::okr::Okr>,
                            }
                            let report = RunReport {
                                run: run.clone(),
                                okr,
                            };
                            println!("{}", serde_json::to_string_pretty(&report)?);
                        } else {
                            println!("\n=== OKR Run Report: {} ===\n", run.name);
                            println!("OKR ID: {}", run.okr_id);
                            println!("Status: {:?}", run.status);

                            if let Some(corr) = &run.correlation_id {
                                println!("Correlation ID: {}", corr);
                            }
                            if let Some(session) = &run.session_id {
                                println!("Session ID: {}", session);
                            }
                            if let Some(checkpoint) = &run.relay_checkpoint_id {
                                println!("Relay Checkpoint ID: {}", checkpoint);
                            }
                            println!("  Started: {}", run.started_at);
                            if let Some(completed) = run.completed_at {
                                println!("Completed: {}", completed);
                            }

                            if !run.kr_progress.is_empty() {
                                println!("\nKR Progress:");
                                for (kr_id, progress) in &run.kr_progress {
                                    println!("  {}: {:.1}%", kr_id, progress * 100.0);
                                }
                            }

                            // Show the parent OKR
                            if let Some(okr) = repo.get_okr(run.okr_id).await? {
                                println!("\nParent OKR: {}", okr.title);
                                println!("OKR Status: {:?}", okr.status);
                            }
                        }
                        Ok(())
                    } else {
                        anyhow::bail!("OKR or Run not found: {}", id)
                    }
                }

                _ => {
                    anyhow::bail!("Unknown OKR action: {}", args.action)
                }
            }
        }
        None => {
            // Default: launch TUI
            let project = cli.project;
            tui::run(project).await
        }
    }
}

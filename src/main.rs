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
mod opencode;
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
mod worktree;

use clap::Parser;
use cli::{Cli, Command};
use std::io::IsTerminal;
use swarm::{DecompositionStrategy, SwarmExecutor};
use telemetry::{TOKEN_USAGE, get_persistent_stats};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

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
            .with(tracing_subscriber::fmt::layer())
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
        Some(Command::Worker(args)) => a2a::worker::run(args).await,
        Some(Command::Spawn(args)) => a2a::spawn::run(args).await,
        Some(Command::Config(args)) => cli::config::execute(args).await,
        Some(Command::Swarm(args)) => {
            let executor = SwarmExecutor::new(swarm::SwarmConfig {
                max_subagents: args.max_subagents,
                max_steps_per_subagent: args.max_steps,
                subagent_timeout_secs: args.timeout,
                model: args.model.clone().or_else(|| Some("zai/glm-5".to_string())),
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
        Some(Command::Rlm(args)) => {
            use provider::ProviderRegistry;
            use std::io::Read;

            // Gather content
            let content = if !args.file.is_empty() {
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
                parts.join("\n")
            } else if let Some(ref c) = args.content {
                if c == "-" {
                    let mut stdin_content = String::new();
                    std::io::stdin().read_to_string(&mut stdin_content)?;
                    stdin_content
                } else {
                    c.clone()
                }
            } else {
                anyhow::bail!("Either --file or --content must be provided");
            };

            let input_tokens = rlm::RlmChunker::estimate_tokens(&content);
            tracing::info!(input_tokens, "RLM: Analyzing content");

            // Detect content type
            let content_type = if args.content_type == "auto" {
                rlm::RlmChunker::detect_content_type(&content)
            } else {
                match args.content_type.as_str() {
                    "code" => rlm::ContentType::Code,
                    "logs" => rlm::ContentType::Logs,
                    "conversation" => rlm::ContentType::Conversation,
                    "documents" => rlm::ContentType::Documents,
                    _ => rlm::ContentType::Mixed,
                }
            };

            // Try to use LLM-powered RLM if provider is available
            let registry = ProviderRegistry::from_vault().await.ok();
            let provider_opt = registry.as_ref().and_then(|r| {
                r.get("zai")
                    .or_else(|| r.get("moonshotai"))
                    .or_else(|| r.get("openai"))
            });

            if let Some(provider) = provider_opt {
                // Use LLM-powered RLM with llm_query() support
                tracing::info!("Using LLM-powered RLM analysis");

                let mut executor =
                    rlm::RlmExecutor::new(content.clone(), provider, "glm-5".to_string())
                        .with_verbose(args.verbose);

                let result = executor.analyze(&args.query).await?;

                if args.json {
                    let output = serde_json::json!({
                        "query": args.query,
                        "content_type": format!("{:?}", content_type),
                        "input_tokens": input_tokens,
                        "answer": result.answer,
                        "iterations": result.iterations,
                        "sub_queries": result.sub_queries.len(),
                        "stats": result.stats,
                    });
                    println!("{}", serde_json::to_string_pretty(&output)?);
                } else {
                    println!("## RLM Analysis Result\n");
                    println!("**Query:** {}\n", args.query);
                    println!("**Answer:**\n{}\n", result.answer);
                    println!("---");
                    println!(
                        "*Iterations: {} | Sub-queries: {} | Time: {}ms*",
                        result.iterations,
                        result.sub_queries.len(),
                        result.stats.elapsed_ms
                    );

                    if !result.sub_queries.is_empty() {
                        println!("\n### Sub-queries made:");
                        for (i, sq) in result.sub_queries.iter().enumerate() {
                            println!("{}. {} -> {} tokens", i + 1, sq.query, sq.tokens_used);
                        }
                    }
                }
            } else {
                // Fallback to static REPL-based analysis
                tracing::info!("Using static RLM analysis (no provider available)");

                // For small content, just use chunking
                if input_tokens < 10000 {
                    let hints = rlm::RlmChunker::get_processing_hints(content_type);
                    let output = format!(
                        "## Content Analysis\n\n\
                         *Input: {} tokens, Type: {:?}*\n\n\
                         {}\n\n\
                         ---\n\n\
                         {}",
                        input_tokens,
                        content_type,
                        hints,
                        rlm::RlmChunker::compress(&content, args.max_tokens, None)
                    );

                    if args.json {
                        let result = serde_json::json!({
                            "query": args.query,
                            "content_type": format!("{:?}", content_type),
                            "input_tokens": input_tokens,
                            "output_tokens": rlm::RlmChunker::estimate_tokens(&output),
                            "output": output,
                        });
                        println!("{}", serde_json::to_string_pretty(&result)?);
                    } else {
                        println!("{}", output);
                    }
                } else {
                    // For large content, use REPL-based exploration
                    let repl = rlm::RlmRepl::new(content.clone(), rlm::ReplRuntime::Rust);

                    // Run exploration
                    let exploration = format!(
                        "=== CONTEXT EXPLORATION ===\n\
                         Total: {} chars, {} lines, ~{} tokens\n\
                         Type: {:?}\n\n\
                         === FIRST 30 LINES ===\n{}\n\n\
                         === LAST 50 LINES ===\n{}\n\
                         === END EXPLORATION ===",
                        content.len(),
                        repl.lines().len(),
                        input_tokens,
                        content_type,
                        repl.head(30).join("\n"),
                        repl.tail(50).join("\n")
                    );

                    // Search for relevant patterns
                    let errors = repl.grep("error|Error|ERROR|failed|Failed");
                    let error_summary = if errors.is_empty() {
                        "No errors found".to_string()
                    } else {
                        format!(
                            "Found {} error lines:\n{}",
                            errors.len(),
                            errors
                                .iter()
                                .take(5)
                                .map(|(i, l)| format!("  {}:{}", i, l))
                                .collect::<Vec<_>>()
                                .join("\n")
                        )
                    };

                    let hints = rlm::RlmChunker::get_processing_hints(content_type);
                    let output = format!(
                        "## RLM Analysis (Static Mode)\n\n\
                         **Query:** {}\n\n\
                         *Input: {} tokens, Type: {:?}*\n\n\
                         *Note: No LLM provider available. Connect Vault for full RLM analysis.*\n\n\
                         ---\n\n\
                         ### Exploration\n\n\
                         {}\n\n\
                         ### Error Summary\n\n\
                         {}\n\n\
                         ### Content Hints\n\n\
                         {}\n\n\
                         ### Compressed Content\n\n\
                         {}",
                        args.query,
                        input_tokens,
                        content_type,
                        exploration,
                        error_summary,
                        hints,
                        rlm::RlmChunker::compress(&content, args.max_tokens / 2, None)
                    );

                    if args.json {
                        let result = serde_json::json!({
                            "query": args.query,
                            "content_type": format!("{:?}", content_type),
                            "input_tokens": input_tokens,
                            "llm_mode": false,
                            "output": output,
                        });
                        println!("{}", serde_json::to_string_pretty(&result)?);
                    } else {
                        println!("{}", output);
                    }
                }
            }
            Ok(())
        }
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
                    let server = mcp::McpServer::new_stdio();
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
                    // Run as MCP server but just list tools
                    let _server = mcp::McpServer::new_stdio();
                    // In a real implementation, we'd query a connected server
                    println!("# Available MCP Tools\n");
                    println!("- **run_command**: Execute shell commands");
                    println!("- **read_file**: Read file contents");
                    println!("- **write_file**: Write to files");
                    println!("- **list_directory**: List directory contents");
                    println!("- **search_files**: Search for files");
                    println!("- **grep_search**: Search file contents");
                    Ok(())
                }
                "call" => {
                    let tool = args
                        .tool
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("--tool required for call action"))?;
                    let arguments = args
                        .arguments
                        .as_ref()
                        .map(|s| serde_json::from_str(s))
                        .transpose()?
                        .unwrap_or(serde_json::json!({}));

                    // For now, just show what would be called
                    println!("Would call tool: {} with arguments: {}", tool, arguments);
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
                            let files_str = if exec.files_affected.is_empty() {
                                String::new()
                            } else {
                                format!(
                                    " → {}",
                                    exec.files_affected
                                        .iter()
                                        .map(|f| f.summary())
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
                            println!("- {} #{} ({}ms)", status, exec.id, exec.duration_ms);
                        }
                    }
                }

                if let Some(ref file_path) = args.file {
                    let file_execs = persistent.by_file(file_path);
                    if !file_execs.is_empty() {
                        println!("\n## Changes to '{}'\n", file_path);
                        for exec in file_execs.iter().take(args.limit) {
                            println!(
                                "- **{}** #{} ({}ms)",
                                exec.tool_name, exec.id, exec.duration_ms
                            );
                            for change in &exec.files_affected {
                                if change.path == *file_path {
                                    if let Some((start, end)) = change.lines_affected {
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
            let mgr = WorktreeManager::new(&cwd)?;

            // List what exists
            let worktrees = mgr.list()?;

            if args.dry_run {
                if args.json {
                    let output = serde_json::json!({
                        "dry_run": true,
                        "worktrees_found": worktrees.len(),
                        "worktrees": worktrees.iter().map(|wt| {
                            serde_json::json!({
                                "id": wt.id,
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
                let count = mgr.cleanup_all()?;

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
        None => {
            // Default: launch TUI
            let project = cli.project;
            tui::run(project).await
        }
    }
}

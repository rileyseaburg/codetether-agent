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
mod cli;
mod config;
pub mod mcp;
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
use cli::{A2aArgs, Cli, Command};
use swarm::{DecompositionStrategy, SwarmExecutor};
use telemetry::{TOKEN_USAGE, TOOL_EXECUTIONS, get_persistent_stats};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Check if we're running TUI - if so, redirect logs to file instead of stderr
    let is_tui = matches!(cli.command, Some(Command::Tui(_)));
    
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
                .with(tracing_subscriber::fmt::layer().with_writer(std::sync::Mutex::new(file)).with_ansi(false))
                .init();
        }
        // If file creation fails, just don't log
    } else {
        tracing_subscriber::registry()
            .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    // Initialize HashiCorp Vault connection for secrets
    if let Ok(secrets_manager) = secrets::SecretsManager::from_env().await {
        if secrets_manager.is_connected() {
            tracing::info!("Connected to HashiCorp Vault for secrets management");
        }
        // Store in global
        let _ = secrets::init_from_manager(secrets_manager);
    } else {
        tracing::warn!("HashiCorp Vault not configured - no provider API keys will be available");
        tracing::warn!("Set VAULT_ADDR and VAULT_TOKEN environment variables to connect");
    }

    match cli.command {
        Some(Command::Tui(args)) => {
            let project = args.project.or(cli.project);
            tui::run(project).await
        }
        Some(Command::Serve(args)) => server::serve(args).await,
        Some(Command::Run(args)) => cli::run::execute(args).await,
        Some(Command::Worker(args)) => a2a::worker::run(args).await,
        Some(Command::Config(args)) => cli::config::execute(args).await,
        Some(Command::Swarm(args)) => {
            let executor = SwarmExecutor::new(swarm::SwarmConfig {
                max_subagents: args.max_subagents,
                max_steps_per_subagent: args.max_steps,
                subagent_timeout_secs: args.timeout,
                model: args.model.clone(),
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
            let provider_opt = registry
                .as_ref()
                .and_then(|r| r.get("moonshotai").or_else(|| r.get("openai")));

            if let Some(provider) = provider_opt {
                // Use LLM-powered RLM with llm_query() support
                tracing::info!("Using LLM-powered RLM analysis");

                let mut executor = rlm::RlmExecutor::new(
                    content.clone(),
                    provider,
                    "kimi-k2-0711-preview".to_string(), // Use Kimi K2.5
                )
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
                        .get("moonshotai")
                        .or_else(|| registry.get("openai"))
                        .ok_or_else(|| anyhow::anyhow!("No provider available in Vault"))?;

                    let model = args
                        .model
                        .unwrap_or_else(|| "kimi-k2-0711-preview".to_string());
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
        None => {
            // Default: A2A worker mode
            // Check if we have a server URL from args or environment
            if let Some(server) = cli.server {
                let args = A2aArgs {
                    server,
                    token: cli.token,
                    name: cli.name,
                    codebases: None,
                    auto_approve: "safe".to_string(),
                    email: None,
                    push_url: None,
                };
                a2a::worker::run(args).await
            } else {
                // No server specified, show help
                eprintln!("CodeTether Agent v{}", env!("CARGO_PKG_VERSION"));
                eprintln!();
                eprintln!("Usage: codetether --server <URL> [OPTIONS]");
                eprintln!("   or: codetether tui                  # Interactive terminal mode");
                eprintln!("   or: codetether serve                # Start API server");
                eprintln!("   or: codetether run <message>        # Non-interactive mode");
                eprintln!();
                eprintln!("Environment variables:");
                eprintln!("  CODETETHER_SERVER   A2A server URL");
                eprintln!("  CODETETHER_TOKEN    Authentication token");
                eprintln!();
                eprintln!("HashiCorp Vault (required for API keys):");
                eprintln!("  VAULT_ADDR          Vault server address");
                eprintln!("  VAULT_TOKEN         Vault authentication token");
                eprintln!("  VAULT_MOUNT         KV secrets mount (default: secret)");
                eprintln!("  VAULT_SECRETS_PATH  Path prefix (default: codetether/providers)");
                eprintln!();
                eprintln!("Run 'codetether --help' for full options.");
                std::process::exit(1);
            }
        }
    }
}

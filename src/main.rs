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
mod provider;
pub mod rlm;
pub mod secrets;
mod server;
mod session;
pub mod swarm;
mod tool;
mod tui;

use clap::Parser;
use cli::{A2aArgs, Cli, Command};
use swarm::{DecompositionStrategy, SwarmExecutor};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

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

    let cli = Cli::parse();

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
                println!("Status: {}", if result.success { "SUCCESS" } else { "FAILED" });
                println!("Subtasks: {}", result.subtask_results.len());
                println!("Speedup: {:.1}x", result.stats.speedup_factor);
                println!("Critical Path: {} steps", result.stats.critical_path_length);
                println!("\n{}", result.result);
            }
            Ok(())
        }
        Some(Command::Rlm(args)) => {
            use std::io::Read;
            use provider::ProviderRegistry;
            
            // Gather content
            let content = if !args.file.is_empty() {
                let mut parts = Vec::new();
                for path in &args.file {
                    match std::fs::read_to_string(path) {
                        Ok(content) => parts.push(format!("=== FILE: {} ===\n{}\n=== END FILE ===\n", path.display(), content)),
                        Err(e) => parts.push(format!("=== FILE: {} ===\nError: {}\n=== END FILE ===\n", path.display(), e)),
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
            let provider_opt = registry.as_ref()
                .and_then(|r| r.get("moonshotai").or_else(|| r.get("openai")));

            if let Some(provider) = provider_opt {
                // Use LLM-powered RLM with llm_query() support
                tracing::info!("Using LLM-powered RLM analysis");
                
                let mut executor = rlm::RlmExecutor::new(
                    content.clone(),
                    provider,
                    "kimi-k2-0711-preview".to_string(), // Use Kimi K2.5
                );
                
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
                    println!("*Iterations: {} | Sub-queries: {} | Time: {}ms*",
                        result.iterations,
                        result.sub_queries.len(),
                        result.stats.elapsed_ms
                    );
                    
                    if !result.sub_queries.is_empty() {
                        println!("\n### Sub-queries made:");
                        for (i, sq) in result.sub_queries.iter().enumerate() {
                            println!("{}. {} -> {} tokens", i+1, sq.query, sq.tokens_used);
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
                        input_tokens, content_type, hints, 
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
                        format!("Found {} error lines:\n{}", 
                            errors.len(),
                            errors.iter().take(5).map(|(i, l)| format!("  {}:{}", i, l)).collect::<Vec<_>>().join("\n")
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
                        args.query, input_tokens, content_type, 
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

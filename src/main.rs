//! CodeTether Agent - A2A-native AI coding agent
//!
//! A Rust implementation of an AI coding agent with first-class support for the
//! A2A (Agent-to-Agent) protocol and the CodeTether ecosystem.
//!
//! Running `codetether` without a subcommand opens the interactive TUI.
//! Use the `worker` subcommand to connect an A2A worker to CodeTether.
//!
//! SECURITY: Provider credentials are loaded from HashiCorp Vault first.
//! Local development may also use env/AWS fallback credentials unless
//! `CODETETHER_DISABLE_ENV_FALLBACK=1` is set for Vault-only mode.

use clap::Parser;
use cli::{Cli, Command};
use codetether_agent::{
    a2a, benchmark, bus, cli, config, forage, github_pr, indexer, mcp, mux, provider, ralph,
    server, swarm, tool, tui, worker_server,
};
use swarm::{DecompositionStrategy, ExecutionMode, SwarmExecutor};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod cleanup_cli;
mod startup_crash;
mod startup_vault;
mod worktree_cli;

mod benchmark_cli;
mod native_cli;
mod ralph_cli;
mod rlm_cli;

#[tokio::main(worker_threads = 8)]
async fn main() -> anyhow::Result<()> {
    // Native local commands bypass network credentials and agent startup.
    if let Some(result) = native_cli::early().await {
        return result;
    }
    // Load local .env for developer workflows (e.g. `cargo run` without exported vars).
    // Existing process environment still takes precedence over .env values.
    let _ = dotenvy::dotenv();

    // Harden the process environment so every spawned subprocess (git,
    // credential helpers, ssh) fails fast instead of prompting on the
    // terminal (e.g. "Username for 'https://forgejo...':" freezing the TUI).
    codetether_agent::noninteractive_env::harden();

    // Install process-wide crypto providers before any TLS/JWT usage, then
    // fail closed if FIPS mode is required (fips build or env) but inactive.
    codetether_agent::tls::ensure_rustls_crypto_provider();
    let _ = jsonwebtoken::crypto::aws_lc::DEFAULT_PROVIDER.install_default();
    codetether_agent::tls::require_fips()?;
    let cli = Cli::parse();
    if let Some(result) = native_cli::dispatch(&cli.command).await {
        return result;
    }

    // Check if we're running TUI - if so, redirect logs to file instead of stdout
    // TUI is the default when no subcommand is given.
    let is_tui = matches!(&cli.command, Some(Command::Tui(_)) | None);
    let is_git_credential_helper = matches!(&cli.command, Some(Command::GitCredentialHelper(_)));

    // Initialize tracing
    if is_tui {
        // For TUI, log to file to avoid corrupting the display
        let log_dir = config::Config::data_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("/tmp/codetether-agent"));
        let _ = std::fs::create_dir_all(&log_dir);
        let log_file = tui::app::run::open_tui_log(&log_dir);

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
    } else if is_git_credential_helper {
        // Git parses credential-helper stdout as a key=value protocol. Do not
        // install stdout tracing here, or normal startup logs become credentials.
    } else {
        // Logs go to STDERR, never stdout. stdout is reserved for program and
        // protocol output (MCP stdio JSON-RPC, `codetether run` piped into
        // other tools, etc.); writing logs there corrupts that stream and is
        // the #1 "codetether pollutes stdout" complaint from agents.
        tracing_subscriber::registry()
            .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
            .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
            .init();
    }

    startup_crash::initialize(is_tui).await;

    startup_vault::initialize(&cli.command, is_tui, is_git_credential_helper).await;

    let mcp_control_plane_url = cli.server.clone();
    let mcp_control_plane_token = cli.token.clone();
    let (mcp_project_root, mcp_project_was_explicit) = (cli.project.clone(), cli.project.is_some());

    match cli.command {
        Some(Command::Bonsai(args)) => cli::bonsai::run(args).await,
        Some(Command::Windows(args)) => cli::windows::run(args).await,
        Some(Command::Tui(args)) => {
            let allow_network = args.allow_network;
            let access_mode = args.access_mode;
            let yolo = args.yolo || cli.yolo;
            // A2A peer is on by default. `--no-a2a` opts out entirely.
            // When on, every field auto-picks a sensible default unless the
            // user supplied an explicit override (--a2a-port / --a2a-name /
            // --a2a-peer, etc.).
            let a2a_options = if args.a2a {
                Some(crate::a2a::spawn::SpawnOptions {
                    name: args.a2a_name.clone(),
                    hostname: args.a2a_hostname.clone(),
                    port: args.a2a_port.unwrap_or(0),
                    public_url: args.a2a_public_url.clone(),
                    description: args.a2a_description.clone(),
                    peer: args.a2a_peer.clone(),
                    discovery_interval_secs: args.a2a_discovery_interval_secs,
                    auto_introduce: args.a2a_auto_introduce,
                    mdns: args.a2a_mdns,
                })
            } else {
                None
            };
            let start_new = args.project.as_deref() == Some(std::path::Path::new("new"));
            let project = if start_new {
                cli.project
            } else {
                args.project.or(cli.project)
            };
            if start_new {
                unsafe { std::env::set_var("CODETETHER_TUI_NEW_SESSION", "1") }
            };
            tui::run(
                project,
                allow_network,
                a2a_options,
                access_mode,
                yolo,
                args.session,
            )
            .await
        }
        Some(Command::Mux(args)) => mux::execute(args).await,
        Some(Command::Serve(args)) => server::serve(args).await,
        Some(Command::Run(args)) => cli::run::execute(args).await,
        Some(Command::Pr(args)) => github_pr::run(args).await,
        Some(Command::Approval(args)) => cli::approval::execute(args),
        Some(Command::Clipboard(args)) => cli::clipboard::execute(args),
        Some(Command::Models(args)) => cli::execute_models(args).await,
        Some(Command::Index(args)) => indexer::run(args).await,
        Some(Command::Auth(args)) => cli::auth::execute(args).await,
        Some(Command::Vault(args)) => cli::vault::execute(args).await,
        Some(Command::Connect(args)) => cli::connect::execute(args).await,
        Some(Command::Forage(args)) => forage::execute(args).await,
        Some(Command::Search(args)) => cli::search::execute(args).await,
        Some(Command::Browserctl(args)) => cli::browserctl::execute(args).await,
        Some(Command::Context(args)) => cli::context::execute(args).await,
        Some(Command::Worker(mut args)) => {
            // Auto-load saved credentials if no token provided
            if args.token.is_none()
                && let Some(creds) = cli::auth::load_saved_credentials()
            {
                let target = args.server.trim_end_matches('/');
                let saved = creds.server.trim_end_matches('/');
                if saved == target {
                    tracing::info!(email = %creds.email, server = %saved, "Using saved credentials from `codetether auth login`");
                    args.token = Some(creds.access_token);
                } else {
                    tracing::warn!(saved_server = %saved, target_server = %target, "Ignoring saved credentials for different server");
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
        Some(Command::GitCredentialHelper(args)) => {
            a2a::git_credentials::run_git_credential_helper(&args).await
        }
        Some(Command::Spawn(args)) => a2a::spawn::run(args).await,
        Some(Command::Config(args)) => cli::config::execute(args).await,
        Some(Command::Swarm(args)) => {
            // Attach an agent bus so sub-agents are observable and can
            // collaborate live (thinking/tool output/shared results) — not just
            // run isolated in parallel. Install it as the process-global bus so
            // tools like `bus_inspect` can read swarm activity.
            let bus = bus::AgentBus::new().into_arc();
            bus::set_global(bus.clone());

            let executor = SwarmExecutor::new(swarm::SwarmConfig {
                max_subagents: args.max_subagents,
                max_steps_per_subagent: args.max_steps,
                subagent_timeout_secs: args.timeout,
                model: args.model.clone(),
                execution_mode: ExecutionMode::from_cli_value(&args.execution_mode),
                k8s_pod_budget: args.k8s_pod_budget,
                k8s_subagent_image: args.k8s_image.clone(),
                ..Default::default()
            })
            .with_bus(bus);

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
        Some(Command::Rlm(args)) => rlm_cli::run(args).await,
        Some(Command::Oracle(args)) => cli::oracle::execute(args).await,
        Some(Command::Ralph(args)) => ralph_cli::execute(args).await,
        Some(Command::Mcp(args)) => {
            // Resolve the target MCP server command from either `--command "cmd args"`
            // (flag form) or trailing positional args (e.g. `connect npx -y @x/foo /path`).
            let resolved_command: Option<(String, Vec<String>)> = {
                if !args.command_args.is_empty() {
                    let mut parts = args.command_args.iter().cloned();
                    let cmd = parts.next().unwrap();
                    Some((cmd, parts.collect()))
                } else if let Some(s) = args.command.as_ref() {
                    let parts: Vec<String> = s.split_whitespace().map(str::to_string).collect();
                    if parts.is_empty() {
                        None
                    } else {
                        let mut it = parts.into_iter();
                        let cmd = it.next().unwrap();
                        Some((cmd, it.collect()))
                    }
                } else {
                    None
                }
            };
            match args.action.as_str() {
                "serve" => {
                    tracing::info!("Starting MCP server over stdio...");

                    // Create agent bus + S3 sink so tool calls sync to MinIO
                    let bus = bus::AgentBus::new().into_arc();
                    codetether_agent::bus::s3_sink::spawn_bus_s3_sink(bus.clone());

                    // Try to load provider from Vault so tools like Ralph can execute autonomously
                    let registry = match provider::ProviderRegistry::from_vault().await {
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
                    let resolved_bus_url = if let Some(bus_url) = &args.bus_url {
                        Some(bus_url.clone())
                    } else if let Some(worker_id) = &args.worker_id {
                        let control_plane_url = mcp_control_plane_url.as_deref().ok_or_else(|| {
                            anyhow::anyhow!(
                                "--worker-id requires --server (or CODETETHER_SERVER) so MCP can resolve the worker connection"
                            )
                        })?;
                        Some(
                            mcp::bus_bridge::resolve_worker_bus_url(
                                control_plane_url,
                                worker_id,
                                mcp_control_plane_token.as_deref(),
                            )
                            .await?,
                        )
                    } else if let Some(workspace_id) = &args.workspace_id {
                        let control_plane_url = mcp_control_plane_url.as_deref().ok_or_else(|| {
                            anyhow::anyhow!(
                                "--workspace-id requires --server (or CODETETHER_SERVER) so MCP can resolve the owning worker"
                            )
                        })?;
                        Some(
                            mcp::bus_bridge::resolve_worker_bus_url_for_workspace(
                                control_plane_url,
                                workspace_id,
                                mcp_control_plane_token.as_deref(),
                            )
                            .await?,
                        )
                    } else if let Some(control_plane_url) = mcp_control_plane_url.as_deref() {
                        let project_root = mcp_project_root
                            .clone()
                            .or_else(|| std::env::current_dir().ok());
                        if let Some(project_root) = project_root.as_deref() {
                            if let Some(workspace_id) =
                                mcp::bus_bridge::resolve_workspace_id_from_path(
                                    control_plane_url,
                                    project_root,
                                    mcp_control_plane_token.as_deref(),
                                )
                                .await?
                            {
                                Some(
                                    mcp::bus_bridge::resolve_worker_bus_url_for_workspace(
                                        control_plane_url,
                                        &workspace_id,
                                        mcp_control_plane_token.as_deref(),
                                    )
                                    .await?,
                                )
                            } else if mcp_project_was_explicit {
                                anyhow::bail!(
                                    "No registered workspace matched project '{}'; pass --workspace-id or --worker-id",
                                    project_root.display()
                                );
                            } else {
                                Some(
                                    mcp::bus_bridge::resolve_default_worker_bus_url(
                                        control_plane_url,
                                        mcp_control_plane_token.as_deref(),
                                    )
                                    .await?,
                                )
                            }
                        } else {
                            Some(
                                mcp::bus_bridge::resolve_default_worker_bus_url(
                                    control_plane_url,
                                    mcp_control_plane_token.as_deref(),
                                )
                                .await?,
                            )
                        }
                    } else {
                        None
                    };
                    let server = if let Some(bus_url) = resolved_bus_url {
                        tracing::info!(bus_url = %bus_url, "Connecting MCP server to agent bus");
                        server
                            .with_bus_auth(bus_url, mcp_control_plane_token.clone())
                            .await
                    } else {
                        server
                    };
                    server.run().await?;
                    Ok(())
                }
                "connect" => {
                    let (cmd, cmd_args) = resolved_command.as_ref().ok_or_else(|| {
                        anyhow::anyhow!(
                            "command required for connect action (pass it positionally, e.g. `codetether mcp connect npx -y @modelcontextprotocol/server-filesystem /path`, or via --command)"
                        )
                    })?;
                    let cmd_arg_refs: Vec<&str> = cmd_args.iter().map(String::as_str).collect();

                    tracing::info!("Connecting to MCP server: {} {}", cmd, cmd_args.join(" "));
                    let client = mcp::McpClient::connect_subprocess(cmd, &cmd_arg_refs).await?;

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
                    if let Some((cmd, cmd_args)) = resolved_command.as_ref() {
                        // Connect to external MCP server and list its tools
                        let cmd_arg_refs: Vec<&str> = cmd_args.iter().map(String::as_str).collect();
                        let client = mcp::McpClient::connect_subprocess(cmd, &cmd_arg_refs).await?;
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

                    if let Some((cmd, cmd_args)) = resolved_command.as_ref() {
                        // Call tool on external MCP server
                        let cmd_arg_refs: Vec<&str> = cmd_args.iter().map(String::as_str).collect();
                        let client = mcp::McpClient::connect_subprocess(cmd, &cmd_arg_refs).await?;
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
        Some(Command::Stats(args)) => codetether_telemetry_cli::execute(args),
        Some(Command::Benchmark(args)) => benchmark_cli::execute(args).await,
        Some(Command::Cleanup(args)) => cleanup_cli::run(args).await,
        Some(Command::Worktree(args)) => worktree_cli::run(args).await,
        Some(Command::Moltbook(args)) => codetether_moltbook_cli::execute(args).await,

        // OKR commands
        Some(Command::Okr(args)) => codetether_okr_cli::execute(args).await,
        None => {
            // Default: launch TUI with auto-everything A2A peer enabled.
            // Same defaults as `codetether tui` with no flags.
            let project = cli.project;
            tui::run(
                project,
                false,
                Some(crate::a2a::spawn::SpawnOptions::auto()),
                None,
                cli.yolo,
                None,
            )
            .await
        }
    }
}

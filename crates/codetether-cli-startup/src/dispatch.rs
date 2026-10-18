//! Exhaustive top-level command routing with native handler bindings.
pub use anyhow;
/// Declare CLI dispatch while retaining application-specific execution paths.
///
/// Handlers receive the same arguments, ownership and await behavior as the
/// native command routes. The command enum remains exhaustive; an absent
/// command starts the same TUI handler without explicit TUI arguments.
/// # Usage
/// Supply the function name, CLI type, command enum and named handler paths.
#[macro_export]
macro_rules! define_command_dispatch {
    ($name:ident, $cli:ty, $command:ident; bonsai = $bonsai:path, windows = $windows:path, tui = $tui:path, mux = $mux:path, serve = $serve:path, run = $run:path, pr = $pr:path, approval = $approval:path, clipboard = $clipboard:path, models = $models:path, index = $index:path, auth = $auth:path, vault = $vault:path, connect = $connect:path, forage = $forage:path, search = $search:path, browser = $browser:path, context = $context:path, worker = $worker:path, credential = $credential:path, spawn = $spawn:path, config = $config:path, swarm = $swarm:path, subagent = $subagent:path, rlm = $rlm:path, oracle = $oracle:path, ralph = $ralph:path, mcp = $mcp:path, stats = $stats:path, benchmark = $benchmark:path, cleanup = $cleanup:path, worktree = $worktree:path, moltbook = $moltbook:path, okr = $okr:path) => {
        async fn $name(cli: $cli) -> $crate::dispatch::anyhow::Result<()> {
            let mcp_control_plane_url = cli.server.clone();
            let mcp_control_plane_token = cli.token.clone();
            let (mcp_project_root, mcp_project_was_explicit) =
                (cli.project.clone(), cli.project.is_some());

            match cli.command {
                Some($command::Bonsai(args)) => $bonsai(args).await,
                Some($command::Windows(args)) => $windows(args).await,
                Some($command::Tui(args)) => $tui(Some(args), cli.project, cli.yolo).await,
                Some($command::Mux(args)) => $mux(args).await,
                Some($command::Serve(args)) => $serve(args).await,
                Some($command::Run(args)) => $run(args).await,
                Some($command::Pr(args)) => $pr(args).await,
                Some($command::Approval(args)) => $approval(args),
                Some($command::Clipboard(args)) => $clipboard(args),
                Some($command::Models(args)) => $models(args).await,
                Some($command::Index(args)) => $index(args).await,
                Some($command::Auth(args)) => $auth(args).await,
                Some($command::Vault(args)) => $vault(args).await,
                Some($command::Connect(args)) => $connect(args).await,
                Some($command::Forage(args)) => $forage(args).await,
                Some($command::Search(args)) => $search(args).await,
                Some($command::Browserctl(args)) => $browser(args).await,
                Some($command::Context(args)) => $context(args).await,
                Some($command::Worker(args)) => $worker(args).await,
                Some($command::GitCredentialHelper(args)) => $credential(&args).await,
                Some($command::Spawn(args)) => $spawn(args).await,
                Some($command::Config(args)) => $config(args).await,
                Some($command::Swarm(args)) => $swarm(args).await,
                Some($command::SwarmSubagent(args)) => $subagent(args).await,
                Some($command::Rlm(args)) => $rlm(args).await,
                Some($command::Oracle(args)) => $oracle(args).await,
                Some($command::Ralph(args)) => $ralph(args).await,
                Some($command::Mcp(args)) => {
                    $mcp(
                        args,
                        &mcp_control_plane_url,
                        &mcp_control_plane_token,
                        &mcp_project_root,
                        mcp_project_was_explicit,
                    )
                    .await
                }
                Some($command::Stats(args)) => $stats(args),
                Some($command::Benchmark(args)) => $benchmark(args).await,
                Some($command::Cleanup(args)) => $cleanup(args).await,
                Some($command::Worktree(args)) => $worktree(args).await,
                Some($command::Moltbook(args)) => $moltbook(args).await,

                // OKR commands
                Some($command::Okr(args)) => $okr(args).await,
                None => $tui(None, cli.project, cli.yolo).await,
            }
        }
    };
}

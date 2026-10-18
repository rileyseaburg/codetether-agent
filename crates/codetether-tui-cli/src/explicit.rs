//! Explicit TUI subcommand options and new-session marker.
use crate::Host;
use codetether_a2a_http::peer_startup::SpawnOptions;
use codetether_cli_options::TuiArgs;
use std::path::PathBuf;
pub(super) async fn execute<H: Host>(
    args: TuiArgs,
    root_project: Option<PathBuf>,
    root_yolo: bool,
) -> anyhow::Result<()> {
    let allow_network = args.allow_network;
    let access_mode = args.access_mode;
    let yolo = args.yolo || root_yolo;
    // A2A peer is on by default. `--no-a2a` opts out entirely.
    // When on, every field auto-picks a sensible default unless the
    // user supplied an explicit override (--a2a-port / --a2a-name /
    // --a2a-peer, etc.).
    let a2a_options = if args.a2a {
        Some(SpawnOptions {
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
        root_project
    } else {
        args.project.or(root_project)
    };
    if start_new {
        unsafe { std::env::set_var("CODETETHER_TUI_NEW_SESSION", "1") }
    };
    H::run(
        project,
        allow_network,
        a2a_options,
        access_mode,
        yolo,
        args.session,
    )
    .await
}

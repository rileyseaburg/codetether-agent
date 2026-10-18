//! CLI tracing destinations and append-only TUI diagnostics.
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
mod file;
pub use file::open as open_tui_log;
/// Configure tracing for the selected CLI output mode.
/// # Arguments
/// `is_tui` routes diagnostics to the TUI log; `is_git_credential_helper`
/// suppresses initialization for the credential protocol.
/// # Returns
/// Installs the same process-wide tracing subscriber as the CLI startup path.
/// # Panics
/// Panics if a tracing subscriber was already installed when initialization is attempted.
/// # Examples
/// ```no_run
/// codetether_cli_startup::logging::initialize(false, false);
/// ```
pub fn initialize(is_tui: bool, is_git_credential_helper: bool) {
    // Initialize tracing
    if is_tui {
        // For TUI, log to file to avoid corrupting the display
        let log_dir = codetether_config::Config::data_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("/tmp/codetether-agent"));
        let _ = std::fs::create_dir_all(&log_dir);
        let log_file = open_tui_log(&log_dir);

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
}

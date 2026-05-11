use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum OfflineCommand {
    /// Record an auth flow: redirect chain, Set-Cookie per hop, final URL
    AuthTrace {
        url: String,
        #[arg(long, default_value_t = 10)]
        max_redirects: u8,
    },
    /// Diff two JSON cookie jars (lists of {name, value, attrs})
    CookieDiff { before: PathBuf, after: PathBuf },
    /// Explain a CORS preflight: send OPTIONS, classify Allow-Origin/Methods
    ExplainCors {
        url: String,
        #[arg(long)]
        origin: String,
        #[arg(long, default_value = "GET")]
        method: String,
    },
    /// Capture an HTTP GET (status + headers + body) to a JSON file
    Record {
        url: String,
        #[arg(long)]
        out: PathBuf,
    },
    /// Replay a captured response through a deterministic browser session
    Replay { capture: PathBuf },
}

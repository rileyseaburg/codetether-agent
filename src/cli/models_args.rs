//! Args for the `models` command.

use clap::Parser;

/// Filter options for listing available provider models.
#[derive(Parser, Debug)]
pub struct ModelsArgs {
    /// Filter by provider name
    #[arg(short, long)]
    pub provider: Option<String>,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

//! Configuration management commands

mod set;
mod status;
mod trust;

use super::{ConfigArgs, ConfigCommand};
use crate::config::Config;
use anyhow::Result;

pub async fn execute(args: ConfigArgs) -> Result<()> {
    if let Some(command) = args.command {
        return match command {
            ConfigCommand::Project(args) => trust::execute(args.command),
        };
    }

    if args.show {
        let config = Config::load().await?;
        println!("{}", toml::to_string_pretty(&config)?);
        status::print_effective(&config)?;
        return Ok(());
    }

    if args.init {
        Config::init_default().await?;
        println!("Configuration initialized");
        return Ok(());
    }

    if let Some(kv) = args.set {
        return set::execute(&kv).await;
    }

    // Default: show help
    println!("Use --show, --init, --set key=value, or config project status");
    Ok(())
}

//! Configuration management commands

use super::ConfigArgs;
use crate::config::Config;
use anyhow::Result;

pub async fn execute(args: ConfigArgs) -> Result<()> {
    if args.show {
        let config = Config::load().await?;
        println!("{}", toml::to_string_pretty(&config)?);
        return Ok(());
    }

    if args.init {
        Config::init_default().await?;
        println!("Configuration initialized");
        return Ok(());
    }

    if let Some(kv) = args.set {
        let parts: Vec<&str> = kv.splitn(2, '=').collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid format. Use: --set key=value");
        }
        Config::set(parts[0], parts[1]).await?;
        println!("Set {} = {}", parts[0], parts[1]);
        if parts[0] == "telemetry.crash_reporting" {
            if matches!(
                parts[1].trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            ) {
                println!(
                    "Crash reporting enabled (opt-in). Panic reports include stack traces and runtime metadata. Disable with: codetether config --set telemetry.crash_reporting=false"
                );
            } else if matches!(
                parts[1].trim().to_ascii_lowercase().as_str(),
                "0" | "false" | "no" | "off"
            ) {
                println!("Crash reporting disabled.");
            }
        }
        return Ok(());
    }

    // Default: show help
    println!("Use --show, --init, or --set key=value");
    Ok(())
}

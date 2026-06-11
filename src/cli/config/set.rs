use crate::config::Config;
use anyhow::Result;

pub async fn execute(kv: &str) -> Result<()> {
    let parts: Vec<&str> = kv.splitn(2, '=').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid format. Use: --set key=value");
    }
    Config::set(parts[0], parts[1]).await?;
    println!("Set {} = {}", parts[0], parts[1]);
    print_telemetry_hint(parts[0], parts[1]);
    Ok(())
}

fn print_telemetry_hint(key: &str, value: &str) {
    if key != "telemetry.crash_reporting" {
        return;
    }
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => println!(
            "Crash reporting enabled (opt-in). Panic reports include stack traces and runtime metadata. Disable with: codetether config --set telemetry.crash_reporting=false"
        ),
        "0" | "false" | "no" | "off" => println!("Crash reporting disabled."),
        _ => {}
    }
}

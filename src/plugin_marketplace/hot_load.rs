//! Hot-load MCP plugin servers at runtime.

use std::path::Path;
use std::process::Command;
use anyhow::Result;

/// A hot-loaded plugin instance.
#[derive(Debug)]
pub struct LoadedPlugin {
    pub name: String,
    pub pid: u32,
    pub endpoint: String,
    pub capabilities: Vec<String>,
}

/// Start a plugin MCP server subprocess.
pub fn start_plugin(name: &str, binary_path: &Path, port: u16) -> Result<LoadedPlugin> {
    let child = Command::new(binary_path)
        .env("MCP_PORT", port.to_string())
        .env("PLUGIN_NAME", name)
        .spawn()?;
    let pid = child.id();
    tracing::info!(name, pid, port, "Plugin MCP server started");
    Ok(LoadedPlugin {
        name: name.to_string(),
        pid,
        endpoint: format!("http://127.0.0.1:{port}"),
        capabilities: Vec::new(),
    })
}

/// Stop a running plugin.
pub fn stop_plugin(plugin: &LoadedPlugin) -> Result<()> {
    tracing::info!(name = %plugin.name, pid = %plugin.pid, "Stopping plugin");
    #[cfg(unix)]
    {
        if plugin.pid > i32::MAX as u32 {
            anyhow::bail!("Invalid PID: {}", plugin.pid);
        }
        let ret = unsafe { libc::kill(plugin.pid as i32, libc::SIGTERM) };
        if ret != 0 {
            anyhow::bail!("Failed to send SIGTERM to plugin {}: errno {}", plugin.name, ret);
        }
    }
    Ok(())
}

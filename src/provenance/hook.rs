use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

pub fn install_commit_msg_hook(path: &Path) -> Result<()> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(path)
        .output()
        .context("Failed to resolve git repo root")?;
    if !output.status.success() {
        return Ok(());
    }
    let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if root.is_empty() {
        return Ok(());
    }
    let hook_path = Path::new(&root).join(".git/hooks/commit-msg");
    let script = "#!/bin/sh\nmsg_file=\"$1\"\n[ -n \"$msg_file\" ] || exit 0\nadd_trailer() {\n  key=\"$1\"\n  value=\"$2\"\n  [ -n \"$value\" ] || return 0\n  git interpret-trailers --in-place --if-exists doNothing --trailer \"$key: $value\" \"$msg_file\"\n}\ngit_cfg() {\n  git config --local --get \"$1\" 2>/dev/null || true\n}\nadd_trailer \"CodeTether-Provenance-ID\" \"$CODETETHER_PROVENANCE_ID\"\nadd_trailer \"CodeTether-Origin\" \"$CODETETHER_ORIGIN\"\nadd_trailer \"CodeTether-Agent-Name\" \"$CODETETHER_AGENT_NAME\"\nadd_trailer \"CodeTether-Agent-Identity\" \"$CODETETHER_AGENT_IDENTITY_ID\"\nadd_trailer \"CodeTether-Tenant-ID\" \"$CODETETHER_TENANT_ID\"\nadd_trailer \"CodeTether-Worker-ID\" \"$CODETETHER_WORKER_ID\"\nadd_trailer \"CodeTether-Session-ID\" \"$CODETETHER_SESSION_ID\"\nadd_trailer \"CodeTether-Task-ID\" \"$CODETETHER_TASK_ID\"\nadd_trailer \"CodeTether-Run-ID\" \"$CODETETHER_RUN_ID\"\nadd_trailer \"CodeTether-Attempt-ID\" \"$CODETETHER_ATTEMPT_ID\"\nadd_trailer \"CodeTether-Key-ID\" \"$CODETETHER_KEY_ID\"\nadd_trailer \"CodeTether-GitHub-Installation-ID\" \"$(git_cfg codetether.githubInstallationId)\"\nadd_trailer \"CodeTether-GitHub-App-ID\" \"$(git_cfg codetether.githubAppId)\"\nadd_trailer \"CodeTether-Signature\" \"$CODETETHER_SIGNATURE\"\n";
    fs::write(&hook_path, script).context("Failed to write commit-msg hook")?;
    let mut permissions = fs::metadata(&hook_path)?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&hook_path, permissions).context("Failed to chmod commit-msg hook")?;
    Ok(())
}

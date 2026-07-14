//! Detection of subprocess commands that may block on authentication.

pub(super) fn reason(command: &str) -> Option<&'static str> {
    let lower = command.to_ascii_lowercase();
    let sudo = lower.starts_with("sudo ")
        || lower.contains(";sudo ")
        || lower.contains("&& sudo ")
        || lower.contains("|| sudo ")
        || lower.contains("| sudo ");
    if sudo && !lower.contains("sudo -n") && !lower.contains("sudo --non-interactive") {
        return Some("Command uses sudo without non-interactive mode (-n).");
    }
    let ssh = lower.starts_with("ssh ")
        || lower.contains(";ssh ")
        || lower.starts_with("scp ")
        || lower.contains(";scp ")
        || lower.starts_with("sftp ")
        || lower.contains(";sftp ")
        || lower.contains(" rsync ");
    if ssh && !lower.contains("batchmode=yes") {
        return Some("SSH-family command may prompt for a password (add -o BatchMode=yes).");
    }
    (lower.starts_with("su ")
        || lower.contains(";su ")
        || lower.contains(" passwd ")
        || lower.starts_with("passwd")
        || lower.contains("ssh-add"))
    .then_some("Command is interactive and may require a password prompt.")
}

pub fn commit_msg_hook_script() -> &'static str {
    "#!/bin/sh
msg_file=\"$1\"
[ -n \"$msg_file\" ] || exit 0
add_trailer() {
  key=\"$1\"
  value=\"$2\"
  [ -n \"$value\" ] || return 0
  git interpret-trailers --in-place --if-exists doNothing --trailer \"$key: $value\" \"$msg_file\"
}
git_cfg() {
  git config --local --get \"$1\" 2>/dev/null || true
}
add_trailer \"CodeTether-Provenance-ID\" \"$CODETETHER_PROVENANCE_ID\"
add_trailer \"CodeTether-Origin\" \"$CODETETHER_ORIGIN\"
add_trailer \"CodeTether-Agent-Name\" \"$CODETETHER_AGENT_NAME\"
add_trailer \"CodeTether-Agent-Identity\" \"$CODETETHER_AGENT_IDENTITY_ID\"
add_trailer \"CodeTether-Tenant-ID\" \"$CODETETHER_TENANT_ID\"
add_trailer \"CodeTether-Worker-ID\" \"$CODETETHER_WORKER_ID\"
add_trailer \"CodeTether-Session-ID\" \"$CODETETHER_SESSION_ID\"
add_trailer \"CodeTether-Task-ID\" \"$CODETETHER_TASK_ID\"
add_trailer \"CodeTether-Run-ID\" \"$CODETETHER_RUN_ID\"
add_trailer \"CodeTether-Attempt-ID\" \"$CODETETHER_ATTEMPT_ID\"
add_trailer \"CodeTether-Key-ID\" \"$CODETETHER_KEY_ID\"
add_trailer \"CodeTether-GitHub-Installation-ID\" \"$(git_cfg codetether.githubInstallationId)\"
add_trailer \"CodeTether-GitHub-App-ID\" \"$(git_cfg codetether.githubAppId)\"
add_trailer \"CodeTether-Signature\" \"$CODETETHER_SIGNATURE\"
"
}

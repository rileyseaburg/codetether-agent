#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
# Reject every non-read-only systemctl invocation; no real services are touched.
systemctl() {
  case "$*" in
    '--user show codetether-spotlessbinco-thinker.service -p LoadState --value')
      printf '%s\n' "$chat_state" ;;
    'show codetether-ubuntu-dev.service -p LoadState --value')
      printf '%s\n' "$worker_state" ;;
    *) echo 'Unexpected systemctl mutation' >&2; exit 99 ;;
  esac
}
script="$root/scripts/install-dev-restart-services.sh"
chat_state=loaded worker_state=loaded
output=$(source "$root/scripts/install-dev-restart.sh")
grep -Fq 'Hey! Installed. Now run this script' <<<"$output"
grep -Fxq "  $script" <<<"$output"
grep -Fq 'CodeTether Chat backend (codetether-spotlessbinco-thinker.service)' <<<"$output"
grep -Fq 'System worker (codetether-ubuntu-dev.service; needs sudo)' <<<"$output"
grep -Fq 'No services were restarted automatically.' <<<"$output"
[[ -x "$script" ]]
chat_state=loaded worker_state=not-found
output=$(source "$root/scripts/install-dev-restart.sh")
grep -Fxq "  $script" <<<"$output"
grep -Fq 'CodeTether Chat backend' <<<"$output"
[[ "$output" != *'System worker'* ]]
chat_state=not-found worker_state=not-found
output=$(source "$root/scripts/install-dev-restart.sh")
[[ "$output" != *'install-dev-restart-services.sh'* ]]
systemctl() { return 1; }
output=$(source "$root/scripts/install-dev-restart.sh")
[[ "$output" != *'install-dev-restart-services.sh'* ]]
grep -Fq 'For other workers' <<<"$output"
unset -f systemctl
output=$(PATH=/nonexistent source "$root/scripts/install-dev-restart.sh")
[[ "$output" == *'For other workers'* ]]
echo 'mocked local: restart guidance checks passed'

#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
fixture=$(mktemp -d)
trap 'printf "Mocked restart evidence retained: %s\n" "$fixture"' EXIT
mkdir -p "$fixture/bin" "$fixture/home/.cargo/bin"
installed="$fixture/home/.cargo/bin/codetether"
printf '#!/usr/bin/env bash\necho codetether 0.0.0-test\n' > "$installed"
# Fake systemctl and sudo shadow the real ones: no service or root-owned file is touched.
cat > "$fixture/bin/systemctl" <<'EOF'
#!/usr/bin/env bash
printf 'systemctl %s\n' "$*" >> "$MOCK_LOG"
case "$*" in
  '--user show '*' -p LoadState --value') echo "$MOCK_CHAT_STATE" ;;
  'show '*' -p LoadState --value') echo "$MOCK_WORKER_STATE" ;;
  *'-p ExecStart --value') echo "{ path=$MOCK_EXEC_PATH ; argv[]=$MOCK_EXEC_PATH serve ; }" ;;
esac
EOF
cat > "$fixture/bin/sudo" <<'EOF'
#!/usr/bin/env bash
printf 'sudo %s\n' "$*" >> "$MOCK_LOG"
[ "$1" = install ] && exit "${MOCK_INSTALL_EXIT:-0}"
exit 0
EOF
chmod +x "$installed" "$fixture/bin/systemctl" "$fixture/bin/sudo"
export HOME="$fixture/home" PATH="$fixture/bin:/usr/bin:/bin"
export MOCK_EXEC_PATH="$installed" MOCK_INSTALL_EXIT=0
run() { # run <case-name>; sets $status, writes $fixture/<case>.{log,out}
  export MOCK_LOG="$fixture/$1.log"
  : > "$MOCK_LOG"
  status=0
  bash "$root/scripts/install-dev-restart-services.sh" > "$fixture/$1.out" 2>&1 || status=$?
}
line() { grep -nF -- "$1" "$MOCK_LOG" | head -1 | cut -d: -f1; }
absent() { # absent <text> <file>: a bare `! grep` would not trip errexit
  if grep -Fq -- "$1" "$2"; then echo "unexpected '$1' in $2" >&2; exit 1; fi
}

export MOCK_CHAT_STATE=loaded MOCK_WORKER_STATE=loaded
run both
[[ "$status" == 0 ]]
grep -Fxq "sudo install -m755 $installed /opt/codetether-worker/bin/codetether" "$MOCK_LOG"
grep -Fxq 'sudo systemctl status codetether-ubuntu-dev.service --no-pager' "$MOCK_LOG"
grep -Fxq 'systemctl --user status codetether-spotlessbinco-thinker.service --no-pager' "$MOCK_LOG"
grep -Fq "ExecStart={ path=$installed" "$fixture/both.out"
absent 'WARNING' "$fixture/both.out"
# Worker is updated before its restart, and the chat backend restarts last.
install_at=$(line 'sudo install -m755')
worker_at=$(line 'sudo systemctl restart codetether-ubuntu-dev.service')
chat_at=$(line 'systemctl --user restart codetether-spotlessbinco-thinker.service')
(( install_at < worker_at && worker_at < chat_at ))

export MOCK_WORKER_STATE=not-found
run chat-only
[[ "$status" == 0 ]]
absent 'sudo ' "$MOCK_LOG"
grep -Fq 'systemctl --user restart' "$MOCK_LOG"

export MOCK_CHAT_STATE=not-found
run neither
[[ "$status" == 0 ]]
absent restart "$MOCK_LOG"

export MOCK_CHAT_STATE=loaded MOCK_WORKER_STATE=loaded MOCK_INSTALL_EXIT=1
run install-fails
[[ "$status" == 1 ]]
absent 'sudo systemctl restart' "$MOCK_LOG"
grep -Fq 'systemctl --user restart' "$MOCK_LOG"
grep -Fq 'FAILED: install' "$fixture/install-fails.out"

export MOCK_INSTALL_EXIT=0 MOCK_EXEC_PATH=/opt/pinned/codetether
run pinned
[[ "$status" == 0 ]]
grep -Fq 'is pinned to /opt/pinned/codetether' "$fixture/pinned.out"

rm "$installed"
run missing-binary
[[ "$status" == 1 ]]
[[ ! -s "$MOCK_LOG" ]]
echo 'mocked local: restart-services script checks passed'

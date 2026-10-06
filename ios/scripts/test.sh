#!/bin/bash
# Focused simulator suite; retain the log and xcresult on amac.
set -euo pipefail
export PATH="/opt/homebrew/bin:$PATH"
cd "${IOS_ROOT:-$HOME/CodeTether-iOS}"
destination="${IOS_TEST_DESTINATION:-platform=iOS Simulator,id=C2DB3AC5-293F-4CAD-95DF-13221ECD0509}"
evidence="$HOME/CodeTether-iOS-evidence/camera-tests-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$evidence"
printf 'Evidence: %s\n' "$evidence"
xcodegen generate
set +e
xcodebuild test -project CodeTether.xcodeproj -scheme CodeTether \
  -destination "$destination" -derivedDataPath build-simulator \
  -resultBundlePath "$evidence/tests.xcresult" \
  >"$evidence/test.log" 2>&1
status=$?
set -e
printf '%s\n' "$status" >"$evidence/exit-status.txt"
tail -25 "$evidence/test.log"
exit "$status"
#!/bin/bash
# Isolated mocked-local Screen tests; never target/relaunch the physical iPhone.
set -uo pipefail
export PATH="/opt/homebrew/bin:$PATH"
root="$HOME/CodeTether-iOSCheck-20261009T052632Z"
attempt="${1:-01}"
cd "$root" || exit 1
mkdir -p results
xcodegen generate >"results/project-$attempt.log" 2>&1 || exit 1
xcodebuild test -project CodeTether.xcodeproj -scheme CodeTether \
  -destination 'platform=iOS Simulator,id=BCB7652D-32A7-4368-9A75-945A17957EB2' \
  -derivedDataPath "$HOME/CodeTether-ScreenTest-20261009T051600Z/build-simulator" \
  -resultBundlePath "results/screen-$attempt.xcresult" \
  -parallel-testing-enabled NO \
  -only-testing:CodeTetherTests/ScreenLifecycleTests \
  -only-testing:CodeTetherTests/ScreenSSETests \
  -only-testing:CodeTetherTests/ScreenQuestionFlowTests \
  -only-testing:CodeTetherTests/ScreenQuestionCancellationTests \
  -only-testing:CodeTetherTests/ScreenReplyFlowTests \
  -only-testing:CodeTetherTests/ScreenQuestionHTTPTests \
  >"results/screen-$attempt.log" 2>&1
status=$?
printf '%s\n' "$status" >"results/screen-$attempt.exit.txt"
xcrun xcresulttool get test-results summary \
  --path "results/screen-$attempt.xcresult" \
  >"results/screen-$attempt-summary.json" 2>"results/summary-$attempt.log"
tail -70 "results/screen-$attempt.log"
printf 'Artifacts: %s/results\n' "$root"
exit "$status"

# Uses only in-memory fixtures / URLProtocol stubs. No Vault credentials,
# physical device bootstrap, installation, stream reset, or relay restart.
# The original source archive, every result bundle and every attempt remain.
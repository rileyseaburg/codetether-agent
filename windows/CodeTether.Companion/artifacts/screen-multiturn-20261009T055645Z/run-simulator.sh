#!/bin/bash
# Mocked-local verification only; never select the paired physical iPhone.
set -uo pipefail
export PATH="/opt/homebrew/bin:$PATH"
root="$HOME/CodeTether-ScreenMultiTurn-20261009T055645Z"
attempt="${1:-01}"
[[ "$attempt" =~ ^[0-9][0-9]$ ]] || exit 2
cd "$root" || exit 1
mkdir -p results
test ! -e "results/screen-$attempt.log" || exit 2
test ! -e "results/screen-$attempt.xcresult" || exit 2
xcodegen generate >"results/project-$attempt.log" 2>&1 || exit 1
xcodebuild test -project CodeTether.xcodeproj -scheme CodeTether \
  -destination 'platform=iOS Simulator,id=BCB7652D-32A7-4368-9A75-945A17957EB2' \
  -derivedDataPath "$root/build-simulator" \
  -resultBundlePath "results/screen-$attempt.xcresult" \
  -parallel-testing-enabled NO \
  -only-testing:CodeTetherTests/ScreenConfigurationTests \
  -only-testing:CodeTetherTests/ScreenStreamRetryTests \
  -only-testing:CodeTetherTests/ScreenReconnectTests \
  -only-testing:CodeTetherTests/ScreenMultiTurnTests \
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
exit "$status"

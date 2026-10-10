#!/bin/bash
# Run only Screen tests on the dedicated simulator, keeping signing enabled.
set -uo pipefail
root="$HOME/CodeTether-ScreenTest-20261009T051600Z"
export PATH="/opt/homebrew/bin:$PATH"
cd "$root" || exit 1
mkdir -p results
xcodebuild test -project CodeTether.xcodeproj -scheme CodeTether \
  -destination 'platform=iOS Simulator,id=BCB7652D-32A7-4368-9A75-945A17957EB2' \
  -derivedDataPath build-simulator -resultBundlePath results/screen-tests-01.xcresult \
  -only-testing:CodeTetherTests/ScreenLifecycleTests \
  -only-testing:CodeTetherTests/ScreenSSETests \
  >results/screen-tests-01.log 2>&1
status=$?
printf '%s\n' "$status" >results/screen-tests-01.exit.txt
tail -50 results/screen-tests-01.log
printf 'Artifacts: %s/results\n' "$root"
exit "$status"

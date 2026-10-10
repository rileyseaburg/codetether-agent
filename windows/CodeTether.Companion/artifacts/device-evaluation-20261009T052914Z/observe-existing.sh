#!/bin/bash
# Attach the observation-only bridge to the existing phone session.
# Xcode uses the hardware UDID, not devicectl's CoreDevice UUID.
set -euo pipefail
cd "$HOME/CodeTether-DeviceEval-20261009T052914Z"
attempt="$1"
[[ "$attempt" =~ ^[0-9]{2}$ ]] || exit 2
base="results/manual-$attempt"
[[ ! -e "$base.log" && ! -e "$base.xcresult" ]] || exit 2
set +e
xcodebuild test-without-building -xctestrun manual.xctestrun \
  -destination 'platform=iOS,id=00008110-001879C23E0A401E' \
  -parallel-testing-enabled NO -resultBundlePath "$base.xcresult" \
  >"$base.log" 2>&1
code=$?
printf '%s\n' "$code" >"$base.exit.txt"
exit "$code"
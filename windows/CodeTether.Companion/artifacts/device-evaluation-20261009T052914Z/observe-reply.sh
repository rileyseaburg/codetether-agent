#!/bin/bash
# Use only the test-runner package; do not install or launch the owner app.
set -euo pipefail
cd "$HOME/CodeTether-DeviceEval-20261009T052914Z"
base=results/reply-observation-04
[[ ! -e "$base.log" && ! -e "$base.xcresult" ]] || exit 2
xcrun devicectl device install app --device 4745B27B-2B68-5164-A053-B869B07EE2CA \
  "$PWD/ReplyObserver-04.app" --timeout 30 > "$base-install.log" 2>&1
set +e
xcodebuild test-without-building -xctestrun reply-observation.xctestrun \
  -destination 'platform=iOS,id=00008110-001879C23E0A401E' \
  -parallel-testing-enabled NO -resultBundlePath "$base.xcresult" > "$base.log" 2>&1
code=$?
printf '%s\n' "$code" > "$base.exit.txt"
set -e
xcrun devicectl device copy from --device 4745B27B-2B68-5164-A053-B869B07EE2CA \
  --domain-type appDataContainer --domain-identifier run.codetether.CodeTetherUITests.xctrunner \
  --source Documents/reply-observation-04.json --destination "$base.json" --timeout 20 --quiet
cat "$base.json"
exit "$code"
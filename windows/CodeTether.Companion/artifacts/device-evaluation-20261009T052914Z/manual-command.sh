#!/bin/bash
# Deliver one explicit command and fetch its status-only response.
set -euo pipefail
cd "$HOME/CodeTether-DeviceEval-20261009T052914Z"
id="$1"
number=$((10#$id))
device=4745B27B-2B68-5164-A053-B869B07EE2CA
runner=run.codetether.CodeTetherUITests.xctrunner
# devicectl skips older sources even when command contents differ.
# Refresh only the selected command; never reissue a typing command implicitly.
touch "command-$id.json"
xcrun devicectl device copy to --device "$device" --domain-type appDataContainer \
  --domain-identifier "$runner" --source "command-$id.json" --destination Documents/command.json --quiet
for attempt in {1..20}; do
  sleep 1
  if xcrun devicectl device copy from --device "$device" --domain-type appDataContainer \
    --domain-identifier "$runner" --source "Documents/response-$number.json" \
    --destination "results/response-$number.json" --quiet 2>/dev/null; then
    cat "results/response-$number.json"; printf '\n'; exit 0
  fi
done
printf 'No response for command %s; execution is unconfirmed.\n' "$number" >&2
exit 1
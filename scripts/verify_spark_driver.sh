#!/usr/bin/env bash
set -euo pipefail

driver_pod=${1:?driver pod name is required}
shift
submit_log=/tmp/spark-submit.log
set +e
"$@" 2>&1 | tee "$submit_log"
submit_status=${PIPESTATUS[0]}
set -e
if [[ $submit_status -ne 0 ]] || ! grep -q 'phase: Succeeded' "$submit_log"; then
    echo "Spark driver $driver_pod did not succeed" >&2
    exit 1
fi

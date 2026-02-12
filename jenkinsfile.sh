
#!/usr/bin/env bash
set -euo pipefail

JENKINS_USER="rileyseaburg"
JENKINS_API_KEY="118e6cdfef989806defeee7b8b70bbac6f"
JENKINS_URL="http://localhost:8080"
JOB="codetether-agent"
SSH_PASS='W!nterSpr!ng20@6'
SSH_HOST="root@192.168.50.134"
CT_ID=132

run_jenkins() {
  sshpass -p "$SSH_PASS" ssh -o StrictHostKeyChecking=no "$SSH_HOST" \
    "pct exec $CT_ID -- bash -c \"$1\""
}

case "${1:-status}" in
  scan)
    echo "==> Triggering multibranch scan..."
    run_jenkins "curl -s -o /dev/null -w 'HTTP %{http_code}\n' -X POST \
      '${JENKINS_URL}/job/${JOB}/indexing/build' \
      --user '${JENKINS_USER}:${JENKINS_API_KEY}'"
    echo "==> Scan triggered. Waiting 15s for indexing..."
    sleep 15
    echo "==> Discovered branches/tags:"
    run_jenkins "ls /var/lib/jenkins/jobs/${JOB}/branches/"
    ;;

  fix-interval)
    echo "==> Reducing scan interval from 24h to 5 minutes..."
    sshpass -p "$SSH_PASS" ssh -o StrictHostKeyChecking=no "$SSH_HOST" \
      "pct exec $CT_ID -- sed -i 's|<interval>86400000</interval>|<interval>300000</interval>|' /var/lib/jenkins/jobs/${JOB}/config.xml"
    echo "==> Reloading Jenkins config..."
    run_jenkins "curl -s -o /dev/null -w 'HTTP %{http_code}\n' -X POST \
      '${JENKINS_URL}/reload' \
      --user '${JENKINS_USER}:${JENKINS_API_KEY}'"
    echo "==> Done. Scan interval is now 5 minutes."
    ;;

  log)
    echo "==> Last indexing log:"
    run_jenkins "tail -30 /var/lib/jenkins/jobs/${JOB}/indexing/indexing.log"
    ;;

  status)
    echo "==> Discovered branches/tags:"
    run_jenkins "ls /var/lib/jenkins/jobs/${JOB}/branches/"
    echo "--- INDEXING ---"
    run_jenkins "ls /var/lib/jenkins/jobs/${JOB}/indexing/"
    ;;

  builds)
    echo "==> Recent builds:"
    run_jenkins "for d in /var/lib/jenkins/jobs/${JOB}/branches/*/builds/*/; do
      [ -f \"\${d}build.xml\" ] && echo \"\$(basename \$(dirname \$(dirname \$d))): build \$(basename \$d)\";
    done | sort | tail -20"
    ;;

  *)
    echo "Usage: $0 {scan|fix-interval|log|status|builds}"
    exit 1
    ;;
esac


#!/usr/bin/env bash
# Seed a local dev Vault with Bedrock credentials from an AWS profile.
#
# Usage:
#   ./seed-bedrock.sh                     # uses AWS_PROFILE or "default"
#   AWS_PROFILE=prod ./seed-bedrock.sh
#   ./seed-bedrock.sh my-profile
#
# Requirements:
#   - Vault container running (see docker-compose.yml in this directory)
#   - ~/.aws/credentials with the selected profile
#   - `vault` CLI installed locally OR fall back to `docker exec` (auto)

set -euo pipefail

PROFILE="${1:-${AWS_PROFILE:-default}}"
CREDS_FILE="${AWS_SHARED_CREDENTIALS_FILE:-$HOME/.aws/credentials}"
CONFIG_FILE="${AWS_CONFIG_FILE:-$HOME/.aws/config}"

export VAULT_ADDR="${VAULT_ADDR:-http://127.0.0.1:8200}"
export VAULT_TOKEN="${VAULT_TOKEN:-${VAULT_DEV_ROOT_TOKEN:-codetether-dev-root}}"
MOUNT="${VAULT_MOUNT:-secret}"
PATH_PREFIX="${VAULT_SECRETS_PATH:-codetether/providers}"

if [[ ! -f "$CREDS_FILE" ]]; then
  echo "ERROR: AWS credentials file not found: $CREDS_FILE" >&2
  exit 1
fi

# Extract key=value for a given [profile] section from an INI file.
# Usage: ini_get <file> <section> <key>
ini_get() {
  local file="$1" section="$2" key="$3"
  awk -v section="[$section]" -v key="$key" '
    $0 == section { in_section = 1; next }
    /^\[/         { in_section = 0 }
    in_section && $0 ~ "^[[:space:]]*"key"[[:space:]]*=" {
      sub(/^[^=]*=[[:space:]]*/, "")
      gsub(/[[:space:]]+$/, "")
      print
      exit
    }
  ' "$file"
}

AWS_KEY_ID=$(ini_get "$CREDS_FILE" "$PROFILE" "aws_access_key_id")
AWS_SECRET=$(ini_get "$CREDS_FILE" "$PROFILE" "aws_secret_access_key")
AWS_SESSION_TOKEN=$(ini_get "$CREDS_FILE" "$PROFILE" "aws_session_token" || true)

if [[ -z "$AWS_KEY_ID" || -z "$AWS_SECRET" ]]; then
  echo "ERROR: profile [$PROFILE] missing aws_access_key_id or aws_secret_access_key in $CREDS_FILE" >&2
  exit 1
fi

# Region: config file uses [profile NAME] (or [default]) convention.
REGION_SECTION="profile $PROFILE"
[[ "$PROFILE" == "default" ]] && REGION_SECTION="default"
AWS_REGION=$(
  [[ -f "$CONFIG_FILE" ]] && ini_get "$CONFIG_FILE" "$REGION_SECTION" "region" || true
)
AWS_REGION="${AWS_REGION:-${AWS_REGION_OVERRIDE:-us-east-1}}"

echo "Seeding Vault at $VAULT_ADDR"
echo "  profile : $PROFILE"
echo "  region  : $AWS_REGION"
echo "  path    : $MOUNT/$PATH_PREFIX/bedrock"

# Pick a vault invocation: local CLI if available, else exec into the container.
if command -v vault >/dev/null 2>&1; then
  VAULT_CMD=(vault)
elif docker ps --format '{{.Names}}' | grep -q '^codetether-vault$'; then
  VAULT_CMD=(docker exec -e "VAULT_ADDR=http://127.0.0.1:8200" -e "VAULT_TOKEN=$VAULT_TOKEN" codetether-vault vault)
else
  echo "ERROR: neither 'vault' CLI nor 'codetether-vault' container available" >&2
  exit 1
fi

# Ensure the KV v2 mount exists (idempotent — ignore "already in use").
"${VAULT_CMD[@]}" secrets enable -path="$MOUNT" kv-v2 2>/dev/null || true

# Build the kv put args
ARGS=(
  "aws_access_key_id=$AWS_KEY_ID"
  "aws_secret_access_key=$AWS_SECRET"
  "region=$AWS_REGION"
)
if [[ -n "${AWS_SESSION_TOKEN:-}" ]]; then
  ARGS+=("aws_session_token=$AWS_SESSION_TOKEN")
fi

"${VAULT_CMD[@]}" kv put "$MOUNT/$PATH_PREFIX/bedrock" "${ARGS[@]}" >/dev/null

echo
echo "OK — Bedrock credentials stored."
echo
echo "Verify:"
echo "  VAULT_ADDR=$VAULT_ADDR VAULT_TOKEN=$VAULT_TOKEN vault kv get $MOUNT/$PATH_PREFIX/bedrock"
echo
echo "Use with codetether:"
cat <<EOF
  export VAULT_ADDR="$VAULT_ADDR"
  export VAULT_TOKEN="$VAULT_TOKEN"
  export VAULT_MOUNT="$MOUNT"
  export VAULT_SECRETS_PATH="$PATH_PREFIX"
  # Optional: force Vault-only (no env fallback)
  # export CODETETHER_DISABLE_ENV_FALLBACK=1
  codetether models
EOF

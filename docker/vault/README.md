# Local Vault for CodeTether Bedrock

A one-shot development setup that runs HashiCorp Vault in Docker and seeds
it with Bedrock credentials from your local AWS profile.

## Usage

```bash
# 1. Start Vault in dev mode
cd docker/vault
docker compose up -d

# 2. Seed with your default AWS profile (reads ~/.aws/credentials + ~/.aws/config)
./seed-bedrock.sh

# Or pick a specific profile
./seed-bedrock.sh prod
AWS_PROFILE=prod ./seed-bedrock.sh
```

## Use with codetether

```bash
export VAULT_ADDR=http://127.0.0.1:8200
export VAULT_TOKEN=codetether-dev-root
export VAULT_MOUNT=secret
export VAULT_SECRETS_PATH=codetether/providers

# Optional: forbid any env-var credential fallback
# export CODETETHER_DISABLE_ENV_FALLBACK=1

codetether models
codetether run --model claude-opus-4-7 "hello"
```

## Secret layout

The seed script writes to `secret/codetether/providers/bedrock` with these
fields (matching the `from_vault()` schema in `src/provider/mod.rs`):

| Field                   | Source                       |
| ----------------------- | ---------------------------- |
| `aws_access_key_id`     | profile `aws_access_key_id`  |
| `aws_secret_access_key` | profile `aws_secret_access_key` |
| `aws_session_token`     | profile `aws_session_token` (optional) |
| `region`                | `~/.aws/config` region for the profile (fallback `us-east-1`) |

## Overrides

| Env var              | Default                        |
| -------------------- | ------------------------------ |
| `VAULT_ADDR`         | `http://127.0.0.1:8200`        |
| `VAULT_TOKEN`        | `codetether-dev-root`          |
| `VAULT_MOUNT`        | `secret`                       |
| `VAULT_SECRETS_PATH` | `codetether/providers`         |
| `AWS_PROFILE`        | `default`                      |
| `AWS_SHARED_CREDENTIALS_FILE` | `~/.aws/credentials`  |
| `AWS_CONFIG_FILE`    | `~/.aws/config`                |
| `AWS_REGION_OVERRIDE`| fallback region if config omits one |

## Security notes

- This compose file runs Vault in **dev mode** (in-memory storage, unsealed,
  root token printed). Do **not** use it for anything beyond local dev.
- The token `codetether-dev-root` is hard-coded for convenience. Override
  via `VAULT_DEV_ROOT_TOKEN` before `docker compose up` if you share the
  host.
- Seeded secrets are lost when the container is recreated (dev mode =
  in-memory). Re-run `./seed-bedrock.sh` after each restart.

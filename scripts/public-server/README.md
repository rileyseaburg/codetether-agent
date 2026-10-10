# Public CodeTether server

`https://server.codetether.run` forwards through a dedicated Cloudflare Tunnel
to the existing `codetether serve` service on ubuntu-dev, `127.0.0.1:4096`.
The website and the separate `api.codetether.run` A2A service are unchanged.
This is an authenticated API, not an anonymous web dashboard.

See [companion server support](../../docs/companion-server-support.md) for the
screen-analysis relay architecture, authentication boundaries, and migration gaps.

## Live deployment evidence (2026-10-04)

- Tunnel: `codetether-server-ubuntu-dev`,
  `d2074245-2a6b-472a-9391-741d672e0544`; Cloudflare reports `healthy`.
- Proxied CNAME: `server.codetether.run` to that tunnel's `cfargotunnel.com` name.
- Origin: CodeTether `4.7.6-dev.23`; exact running binary hash is in the artifacts.
- HTTPS `/health`: 200; anonymous/invalid-token `/api/version`: 401;
  authenticated `/api/version` and `/api/agent`: 200.
- Evidence: `artifacts/public-server-20261004/`.

## Credentials and lifecycle

The Cloudflare account credential comes from Vault `kv/cloudflare/api-token`.
The server bearer token is stored at Vault
`secret/codetether/endpoints/public-server`, field `token`.
Clients must send it in `Authorization: Bearer <token>`; never put it in a URL.
The previous generated server token was rotated before public routing began.

Local credentials live in `~/.config/codetether-public-server/` (directory 0700,
credential files 0600). Never commit or print them. A systemd drop-in gives the
origin its persistent token. The tunnel uses a token file, not command arguments.
Both user services are enabled, restart automatically, and user lingering is on.
This is a single workstation origin: availability depends on ubuntu-dev staying up.

## Operations

Run from the repository root, as riley, with the managed Vault Agent available:

```bash
bash scripts/public-server/inspect.sh
# Initial setup only: persists auth and RESTARTS the existing origin service.
bash scripts/public-server/secure-origin.sh
bash scripts/public-server/provision-tunnel.sh
bash scripts/public-server/verify.sh artifacts/public-server-manual-check
```

To immediately withdraw public access without stopping the local server:
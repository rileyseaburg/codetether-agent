# Live Forgejo provenance evidence

Captured on 2026-07-18 with the repository-prescribed `forgejo-cli` and the
Vault token at `kv/forgejo/spotlessbinco-agent`. The token was not printed or
persisted.

## Repository commit API

- API: `https://forgejo.quantum-forge.io/api/v1`
- Repository: `riley/spotlessbinco`
- Main commit: `a4c0b15307b0446660a9f1246de3fb98698f8e8e`
- Response path: `.commit.verification`
- Main response: `verified=false`, `reason=gpg.error.not_signed_commit`,
  `signer=null`

## Signed CodeTether commit

- Commit: `1eac554a2d007dc3245f19c3e4546d0bf3d2f72c`
- Forgejo response: `verified=false`, `reason=gpg.error.no_gpg_keys_found`,
  `signer=null`
- Embedded signature fingerprint:
  `B2DB8FD844FE2FCAB6309E87B932D8080A95F13B`

## Trusted-key verification

After importing `config/codetether/author-signing-key.asc` into an isolated
temporary `GNUPGHOME`, `git verify-commit` reported a good signature for the
signed CodeTether commit above. The verified primary fingerprint was
`B2DB8FD844FE2FCAB6309E87B932D8080A95F13B` and the signer identity was
`Riley Seaburg <riley@rileyseaburg.com>`.
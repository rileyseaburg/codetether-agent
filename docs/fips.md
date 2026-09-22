# FIPS 140-3 cryptography

CodeTether Agent can be built to use only a FIPS 140-3 cryptographic module
for TLS. This page describes what that covers and what it does not.

## Default build

- All TLS (HTTP clients, Kubernetes client, WebSockets, QUIC) uses rustls with
  the AWS-LC backend (`aws-lc-rs`).
- `ring` is not in the dependency graph. Check with
  `cargo tree -i ring -e normal,dev` (expect "nothing to print").
- The default build links stock AWS-LC, not the validated module, so it is
  **not** FIPS 140-3 compliant.

## FIPS build

```bash
# Requires CMake, Go, and a C compiler (the AWS-LC FIPS module build needs Go).
cargo build --release --features fips
```

With `--features fips`:

- `aws-lc-rs` links `aws-lc-fips-sys` (the AWS-LC-FIPS 3.0.x module), and
  rustls, rustls-webpki, quinn, and jsonwebtoken all use it.
- rustls installs `default_fips_provider()`, which allows only FIPS-approved
  cipher suites, key-exchange groups, and signature algorithms.
- At startup, `tls::require_fips()` checks three things: the feature is
  compiled in, AWS-LC reports `FIPS_mode() == 1`, and the installed provider
  reports `fips() == true`. If any check fails, the process exits.

A non-FIPS build can also be told to fail closed:
`CODETETHER_REQUIRE_FIPS=1` makes startup exit, because the validated
module is not linked.

## Validation status

Using a validated module does not by itself make a product FIPS compliant.
Check the module's current certificate status on the NIST CMVP list and its
security policy (operating environments, approved services) before claiming
compliance. See the `aws-lc-fips-sys` README and
<https://csrc.nist.gov/projects/cryptographic-module-validation-program>.

## Not yet covered (known gaps)

| Area | Current crypto | Status |
|------|----------------|--------|
| HMAC-SHA256 (provenance signatures, worker proofs, S3 SigV4, sandbox manifests) | RustCrypto `hmac`/`sha2` | Not in the FIPS module; must move to `aws_lc_rs::hmac` |
| SHA-256 for content hashes and IDs | RustCrypto `sha2` | Not security-relevant in most call sites; audit each |
| `tetherscript` (`openssl-tls` feature) | System OpenSSL | Needs the OpenSSL 3 FIPS provider on the host |
| Python A2A server and dashboard | CPython `ssl`/`hashlib`, Node | Need FIPS-validated base images |
| `crates/codetether-rlm` | RustCrypto `sha2` 0.10 | Same as above |

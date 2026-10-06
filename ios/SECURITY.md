# Credential handling

- Server bearer: Vault `secret/codetether/endpoints/public-server`, field `token`.
- Apple API key: Vault `secret/codetether/ios-api-key`; never included in app resources.
- Generated development signing identity: Vault `secret/codetether/ios-signing-development`.
  Certificate ID `R2K43QYVRR`; existing certificates were not revoked.
- Build scripts read Vault through its existing authenticated CLI session.
  JSON travels to the trusted Mac over SSH stdin, not command-line arguments.
- The Apple `.p8` lives in a private temporary directory only during signing.
  The development identity is retained in the Mac login Keychain, with its
  private key also in a mode-0700 signing directory and backed up in Vault.
- The Vault Mac password unlocks the login Keychain. No sudo or Apple account
  password entry is required. Native `security` accepts its password argument
  in process argv; it is never printed or retained in source/build logs.

## iPhone

The app uses only the fixed HTTPS origin, normal TLS trust validation,
ephemeral URLSession storage, no HTTP cookies/cache, and rejects redirects.
The token is a device-only `kSecAttrAccessibleWhenUnlockedThisDeviceOnly`
Keychain item; no token is stored in UserDefaults, bundled assets, or iCloud.
Settings offers a secure replacement field and token removal.

During development installation, a mode-0600 bootstrap file is transferred
into the app container over the paired device connection. On first launch
it is excluded from backup, protected while locked, consumed into Keychain,
and removed. Unlock the phone and finish launch promptly: protection is
applied by the app, not by devicectl, so an interrupted launch leaves the
pending file until the app next opens. Never package it into the `.app`.

Receipts contain endpoint/version, session IDs/tool names or audio route,
volume, duration and timestamps. They contain no bearer token or chat text.
App-switcher content is obscured while inactive. The bearer retains the same
server permissions as the existing Vault token; this app does not invent a
per-user authorization or token-exchange service.

Images are explicitly chosen with the system picker, resized to JPEG, and
uploaded over authenticated HTTPS. Server files are mode0600 in a private
directory. Media reads are restricted to canonical upload/generated-image
paths. Saved chats use existing authenticated server sessions; runtime-only
continuation envelopes are omitted from the display, not from agent context.
Dictation requires iOS permission and never auto-sends recorded text.

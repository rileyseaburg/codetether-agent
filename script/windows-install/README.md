# Windows package-identity setup

The native OCR probe is launched through `System.Diagnostics.Process` from the
writable evidence directory. stdout and stderr are drained independently and
retained as separate files. Native warnings on stderr do not become PowerShell
5.1 terminating errors: only process launch/timeout, nonzero exit, malformed JSON,
or missing readiness flags fail this gate. The probe does not initialize agent
telemetry or provider/Vault startup.

An actual per-user **MSI** is produced by `python3 script/build-windows-msi.py`.
It stages the payload locally before invoking package/OCR setup. See
`script/windows_msi/README.md` for MSI ownership and verification details.

The original Vault configuration prompt is included: `VAULT_ADDR`, hidden
`VAULT_TOKEN` input, user-profile persistence, and default-model discovery.
A dialog is used when MSI has no console; Cancel leaves credentials unchanged.
Token values are not logged or placed in command arguments. As in the original
installer, saved tokens live in the Windows user environment, not encrypted storage.

For an extracted Windows bundle, double-click **Install-CodeTether.cmd**. It runs
the bundled installer with process-local `RemoteSigned`, trusts only those
bundled script files, preserves stronger organization policies, and handles setup.

Run from a **normal, non-administrator** PowerShell on Windows 10 build 19041+ / Windows 11 (x64 or ARM64):

```powershell
.\install.ps1                               # use adjacent bundled codetether.msix or codetether.exe
.\install.ps1 -ExePath .\dist\codetether.exe  # local build; no published release required
.\install.ps1 -MsixPath .\codetether-signed.msix
.\install.ps1 -Version v1.2.3                # example explicit release tag
```

On the normal supported path, users run setup and approve its explained UAC request; there are no manual Python/pip, Tesseract, Rust, SDK installation, or OCR language commands. Enterprise servicing policy, absence of supported/installed OCR languages, disabled aliases, denied UAC, blocked registration, or a failing native readiness probe stop setup explicitly. PowerShell 7 / 32-bit PowerShell automatically hand off to native Windows PowerShell without elevation. Existing Vault/provider credentials are untouched. Optional `-FunctionGemma` / `-FunctionGemmaOnly` remain separate from OCR; gated tokenizer access uses existing `HF_TOKEN`, without prompting for or logging tokens. `-Force` remains accepted; prerequisites are always rechecked.

## Distribution contract

Bundle `install.ps1` and `codetether.exe` beside each other in `dist/windows/`, plus **all top-level `script/windows-install/*.ps1` helpers** in either `dist/windows/script/windows-install/` (repository layout) or `dist/windows/windows-install/` (compact layout). Tests/evidence are not required in release archives. Without options, setup selects adjacent `codetether.msix` first, otherwise `codetether.exe`; it does not download helpers or discover a GitHub release. Explicit `-ExePath`, `-MsixPath`, or `-Version` override bundle selection. Local executable packaging automatically downloads the pinned SDK package. Docker/archive copying belongs to the parent integration, outside this scripts-only change.

For `irm .../install.ps1 | iex`, the bootstrap resolves the release tag once to its immutable Git commit, retrieves that helper tree, and checks each helper's Git blob hash. Helpers never come from mutable `main`. Releases predating them stop with `RELEASE_HAS_NO_WINDOWS_HELPERS`; a checkout/bundled installer plus local executable does not require a release. The initial bootstrap must itself come from a trusted source.

Recognized assets: `codetether-<tag>-<x86_64|aarch64>-pc-windows-<msvc|gnu>.<msix|zip|exe|tar.gz>`. MSIX wins when present; an invalid/untrusted MSIX is a hard failure. Release downloads require GitHub's SHA256 asset digest. Trusted local inputs bypass network discovery, not MSIX signature enforcement.

Release MSIX must use identity `CodeTether.Agent`, application ID `CodeTether`, root executable `codetether.exe`, `Windows.FullTrustApplication`, `runFullTrust`, and the alias shown in `manifest.ps1`. Windows checks release signatures against existing trust; setup never auto-trusts a release-provided certificate. The executable must implement the parent's model/Vault-independent `windows ocr-status --require-ready` contract: JSON boolean `available`, nonzero unless actual process package identity, usable native OCR engine/language, and blank-image recognition are confirmed. These scripts do not implement or substitute for that native probe.

## Local signing and explicit elevation

Packaging uses `Microsoft.Windows.SDK.BuildTools` **10.0.26100.3916**, verified against the SHA512 literal in `sdk.ps1` before extraction/tool execution. The digest was computed from official NuGet bytes, not an independently signed checksum publication; the attempted `.nupkg.sha512` URL returned 404. The archive contains `bin/10.0.26100.0/{x64,arm64}/{makeappx,signtool}.exe`.

The local identity is `CodeTether.Agent.Local`. The private RSA signing key stays **non-exportable in CurrentUser/My**. Both newly created and reused keys must have export disabled according to their Windows CNG/CSP provider; unknown/exportable policies are rejected. Signing uses `signtool /sha1 <thumbprint> /s My`; no PFX export, password argument, or machine private-key store is used. Before UAC, setup displays the public leaf's subject/thumbprint and explains that trusting it permits packages signed by this user's key on the machine.

The one elevated process performs **only** these operations:

1. Add the generated, self-signed, non-CA code-signing leaf's public DER bytes to **LocalMachine/TrustedPeople**, never Root. It rejects CA certificates and certificates without the code-signing EKU.
2. Query/add/confirm Windows `Language.OCR` capabilities. Language preferences are captured from the installing user's profile **before UAC**, not from the administrator's profile. Selection prefers an installed profile recognizer, then any already installed OCR recognizer (matching the parent's runtime selection), then automatically installs supported profile-language capabilities. A same-language regional capability may be selected where no exact capability exists. Empty/unsupported profiles can use an installed recognizer without Windows Update. No profile/installed recognizer produces an explicit error; capability state alone never establishes readiness.

The elevated command contains frozen inline PowerShell and base64-encoded public data. It executes no downloaded script file, SDK tool, app executable, or network helper. **Developer Mode, execution policy, and signature enforcement are unchanged.** Consent/servicing errors stop registration. The elevated error window retains its diagnostic until Enter; servicing details remain in `C:\Windows\Logs\DISM\dism.log`. Reboot-required servicing stops and requests reboot/retry, never declaring ready.

## Alias precedence, readiness, and preservation

`Add-AppxPackage` registers for the original user. Setup verifies identity/version and confirms the family-scoped `WindowsApps/<PackageFamilyName>/codetether.exe` is an **AppExecLink**, not a symlink/bare executable. It runs that alias with `--version`, then **the same packaged alias** with `windows ocr-status --require-ready`. Both exit code zero and JSON boolean `available=true` are required. Failed activation or malformed/missing/false status stops installation; the downloaded bare executable is never used for the probe.

Only after that gate succeeds does setup prepend the family-scoped alias directory to both current and persisted user PATH, verify command resolution, and move the legacy `%LOCALAPPDATA%\codetether\bin\codetether.exe` into retained evidence. Thus the old bare bin cannot precede the packaged alias on user PATH. Conflicting machine PATH commands produce a hard error, not a misleading success. Prior packaged executable/DLL bytes are backed up before upgrades; this is preservation, not automatic rollback of Windows package registration after a failed probe. Local input executables are not moved/deleted.

Evidence remains in `%LOCALAPPDATA%\codetether\install-evidence\<run-id>`: SDK/package, signing leaf public certificate, build/signing logs, registration identity, activation output, **`ocr-status.json` and `ocr-status.stderr.log`**, failure diagnostics, prior PATH, and old executable bytes. Downloaded helpers remain there too. No cleanup deletes failed artifacts. The parent's blank-image probe establishes basic native recognition functionality, not image-recognition accuracy.

## Verification levels

Test logs are retained under repository `artifacts/windows-installer-evidence/`, not in the distribution.

- **static/local:** `pwsh -NoProfile -File script/windows-install/tests/run.ps1` parses scripts, enforces 50 code lines/file, compiles alias-tag interop without invoking Windows, checks trust/key/PATH security contracts, and checks SDK bytes when the retained archive exists.
- **mocked local:** Tests cover bundled no-network resolution, native readiness JSON/exit/error gates, empty profiles and installed alternate recognizers, PE/package contracts, CA rejection, download integrity failures, UAC denial, capability/reboot failures, registration failure, and preservation ordering. Each run retains `evidence/tests-<id>/contract-tests.log` and fixtures.
- **not-run:** Windows SDK tools, certificate creation/trust/key-provider policy, UAC, Appx registration, actual AppExecLink activation, and native OCR readiness/recognition. No Windows host was available. Windows acceptance must cover these operations and error cases before live readiness is claimed.

Microsoft references: [desktop package extensions/aliases](https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/desktop-to-uwp-extensions), [package identity](https://learn.microsoft.com/en-us/windows/apps/desktop/modernize/modernize-packaged-apps), [OcrEngine](https://learn.microsoft.com/en-us/uwp/api/windows.media.ocr.ocrengine).
# Windows: install, verify, then launch

Use Windows 10 build 19041+ or Windows 11. The current GitHub Windows release is **x64**; there is no native ARM64 release asset. Do not confuse installer support for a local ARM64 executable with a published ARM64 download.

## 1. Install or update

Save your work and close CodeTether before updating so Windows can replace its registered package. Open **64-bit Windows PowerShell normally**, not “Run as administrator”, then paste:

```powershell
irm https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main/install.ps1 | iex
```

Wait for setup to finish. Approve only its explained UAC requests for prerequisites/certificate trust. Stop on registration, policy or readiness errors; do not disable organization policy or assume an MSI exit means package activation succeeded.
The PowerShell command is the recommended install/update path. It downloads from GitHub, verifies checksums and registers the per-user package. Downloading the EXE alone is not the package-identity installation.

## 2. Verify registration and command resolution

Paste this **after** setup finishes:

```powershell
Get-AppxPackage -Name 'CodeTether*' | Select-Object Name, Version, Status, PackageFamilyName
Get-Command codetether -All | Select-Object CommandType, Source
codetether --version
```

Compare the CLI version with the release you installed (`4.7.6-dev.6` currently). The locally generated MSIX version is timestamp-based and is **not** the CLI version. A release-page success, package entry, or restarted terminal alone does not prove which binary ran.

## 3. If the command is missing or still old

For the normal local-package installation, test its registered alias explicitly:

```powershell
$packages = @(Get-AppxPackage -Name 'CodeTether.Agent.Local')
if ($packages.Count -ne 1) { throw 'Expected one registered local package. Inspect the installer error/logs before continuing.' }
$alias = Join-Path $env:LOCALAPPDATA "Microsoft\WindowsApps\$($packages[0].PackageFamilyName)\codetether.exe"
if (-not (Test-Path -LiteralPath $alias)) { throw 'The registered app-execution alias is missing. Check the installer logs and Windows app-execution alias settings.' }
& $alias --version
```

If the explicit alias is correct but the bare command is not, the issue is command resolution, not a failed download. Use `& $alias tui` once credentials are configured; do not delete another installation blindly. For a signed MSIX with a different identity, use the `Packaged command:` path printed by its installer instead.
A child installer cannot rewrite an already-open parent terminal's environment. A new Windows Terminal tab can inherit the existing host's old environment; reopening CodeTether is not an alias or token repair.

## 4. Configure credentials and start

Follow [Windows Vault setup and model discovery](install_windows_vault.md). Keep the installer evidence directory printed during setup (`%LOCALAPPDATA%\codetether\install-evidence`); it contains activation/readiness diagnostics, not a request to share your token.
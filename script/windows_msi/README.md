# CodeTether MSI bootstrap

Run `python3 script/build-windows-msi.py` after producing `dist/codetether.exe`.
The builder packages the existing executable, native setup helpers, and original
Vault configuration prompts into a genuine x64 Windows Installer database.
It does not rebuild Rust or merely rename a ZIP.

The MSI is per-user and extracts to LocalAppData. Native package/OCR setup runs
as a **deferred, impersonated** action queued after `InstallFiles` and before
`InstallFinalize`. An immediate action at that sequence would run before the
queued file copies materialize. Verification requires MSI action type 1074.
Failures propagate to MSI; maintenance retries setup. Elevated and
noninteractive first installations are rejected before committing files.
Normal Windows consent is still required for certificate trust and OCR servicing.

The MSI installs the **CodeTether Setup cache**. The actual application is an
MSIX registered for package identity (needed by WinRT OCR). Removing the setup
cache does not remove the application: uninstall CodeTether through Windows Apps.
Shared OCR capabilities and trust certificates are not broadly removed.
Native package/servicing operations are not transactionally owned by MSI, so
MSI rollback is not a promise to undo successful external Windows operations.

The original Vault address/token configuration follows packaged-app readiness.
Token entry is hidden; a dialog supports MSI launches without a console.
Persistence remains in the Windows user environment, as in the original installer.
Tokens are not printed or supplied as process arguments.

`wixl` authoring warnings are rejected. Verification checks actual MSI tables,
user-context action flags, sequencing, same-version upgrade detection, and every
file extracted from the embedded cabinet. Only verified candidates are promoted
to `dist/codetether-windows.msi`; failures remain in timestamped evidence folders.

Evidence is under `artifacts/windows-msi/`. These are static/local package and
mocked setup checks, not live Windows execution. The MSI is not Authenticode-signed;
Windows publisher/security prompts and organizational policy remain authoritative.
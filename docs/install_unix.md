# Linux and macOS: copy-and-paste installation

Use Bash or zsh for these examples. [Forgejo releases](https://forgejo.quantum-forge.io/riley/codetether-agent/releases) provide Linux x86-64, Apple Silicon macOS, and Intel macOS packages. A detected architecture alone does not imply a release asset exists for it. No Rust toolchain or Xcode is needed for these prebuilt packages.

## 1. Install or update

Run as your normal user. The installer requests sudo only if needed for its chosen destination.

```sh
curl -fsSL https://forgejo.quantum-forge.io/riley/codetether-agent/raw/branch/main/install.sh | sh
```

Wait for a successful install message. Release metadata, archives, checksums, and immutable hash-checked setup helpers come from Forgejo. The installer includes published development prereleases, excludes drafts, and requires the archive to match the release checksum manifest. Stop on checksum, extraction or permission errors. It normally installs to `/usr/local/bin`; without sudo it falls back to `~/.local/bin`.

## 2. Verify the executable you will run

```sh
command -v codetether
codetether --version
```

Compare with the tag you installed from [Forgejo releases](https://forgejo.quantum-forge.io/riley/codetether-agent/releases). The pinned example below uses `4.7.6-dev.75`; the automatic installer may select a newer release. If missing or older, do not assume another terminal fixes it. Check the exact destination printed by the installer:

```sh
if [ -x /usr/local/bin/codetether ]; then /usr/local/bin/codetether --version; fi
if [ -x "$HOME/.local/bin/codetether" ]; then "$HOME/.local/bin/codetether" --version; fi
if [ -x "$HOME/.cargo/bin/codetether" ]; then "$HOME/.cargo/bin/codetether" --version; fi
```

Use the explicit path that reports the expected version for subsequent commands. If setup chose `~/.local/bin`, this updates only the current shell's PATH:

```sh
export PATH="$HOME/.local/bin:$PATH"
hash -r
command -v codetether
codetether --version
```

Do not delete another installation blindly. macOS artifacts are unsigned; this guide does not promise notarization or advise disabling Gatekeeper. If macOS rejects execution, retain its exact diagnostic.

## macOS-specific installation

### Choose Apple Silicon or Intel

Check **Apple menu → About This Mac**: “Chip: Apple M…” means Apple Silicon;
“Processor: Intel…” means Intel. `uname -m` normally reports `arm64` or `x86_64`,
respectively, but a Rosetta terminal reports `x86_64` even on Apple Silicon.

| Hardware | Archive suffix | Execution |
| --- | --- | --- |
| Apple Silicon | `aarch64-apple-darwin.tar.gz` | Native ARM64; Rosetta is not required |
| Intel | `x86_64-apple-darwin.tar.gz` | Native Intel |

Prefer a native Terminal on Apple Silicon. Both the automatic installer and the
manual example below detect `sysctl.proc_translated=1` and choose ARM64 when
invoked from a Rosetta terminal. An Intel binary can also run through Rosetta on
Apple Silicon, but that is not a substitute for testing on a physical Intel Mac.

### Manual, pinned-version install without sudo

This installs **v4.7.6-dev.75** into `~/.local/bin`, without changing system
directories or shell profiles. It replaces an existing binary at that exact
destination; inspect `command -v codetether` first if you have another install.
Change `version` only to a published Forgejo tag with the matching Mac asset.
Run the entire block in Terminal:

```sh
(
  set -eu
  version=v4.7.6-dev.75
  case "$(uname -m)" in
    arm64) target=aarch64-apple-darwin ;;
    x86_64)
      if [ "$(sysctl -in sysctl.proc_translated 2>/dev/null || true)" = 1 ]; then
        target=aarch64-apple-darwin
      else
        target=x86_64-apple-darwin
      fi ;;
    *) echo 'Unsupported Mac architecture' >&2; exit 1 ;;
  esac
  base="https://forgejo.quantum-forge.io/riley/codetether-agent/releases/download/$version"
  archive="codetether-$version-$target.tar.gz"
  work=$(mktemp -d)
  printf 'Downloads and checksum evidence: %s\n' "$work"
  curl -fSL "$base/$archive" -o "$work/$archive"
  curl -fSL "$base/SHA256SUMS-$version.txt" -o "$work/SHA256SUMS.txt"
  expected=$(awk -v name="$archive" '$2 == name {print $1; exit}' "$work/SHA256SUMS.txt")
  actual=$(shasum -a 256 "$work/$archive")
  [ "${#expected}" -eq 64 ] && [ "$expected" = "${actual%% *}" ] || {
    echo 'Missing checksum or checksum mismatch; not installing.' >&2
    exit 1
  }
  test "$(tar -tzf "$work/$archive")" = codetether
  tar -xzf "$work/$archive" -C "$work" codetether
  mkdir -p "$HOME/.local/bin"
  install -m 755 "$work/codetether" "$HOME/.local/bin/codetether"
  "$HOME/.local/bin/codetether" --version
)
```

### zsh PATH and architecture checks

For a `~/.local/bin` install, add the following line **once** to `~/.zshrc`:

```sh
export PATH="$HOME/.local/bin:$PATH"
```

Then reload the profile and inspect the executable actually selected:

```sh
source ~/.zshrc
rehash
command -v codetether
type -a codetether
file "$(command -v codetether)"
codetether --version
```

`file` should report a Mach-O `arm64` executable on Apple Silicon or `x86_64`
on Intel. If `Bad CPU type in executable` appears, check the archive target and
hardware; download the correct architecture instead of assuming Rosetta is
required. If an older version appears, fix PATH precedence rather than deleting
an unrelated installation.

### Unsigned downloads and Gatekeeper

These macOS archives are unsigned and not notarized. A matching checksum checks
download integrity, not Apple developer signing or notarization. If macOS blocks
execution, retain its exact message and verify the source and checksum first.
Only if you trust the release, review Apple's per-app **Privacy & Security →
Open Anyway** option when available. Do not disable Gatekeeper globally, remove
quarantine recursively, or bypass a warning that the file is damaged or malicious.

## 3. Configure credentials and launch

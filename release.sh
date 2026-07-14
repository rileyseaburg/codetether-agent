#!/bin/bash

# This script is used to create a tagged release of the codetether-agent. It performs the following steps:
# 1. Reads the current version from Cargo.toml
# 2. Asks whether this is a major, minor, or patch bump
# 3. Commits any pending changes via commit.sh (AI-generated commit message)
# 4. Updates the version number in Cargo.toml
# 5. Runs release verification to catch obvious failures before tagging
# 6. Commits the version bump and pushes to the main branch
# 7. Generates AI release notes from commits since the last tag
# 8. Creates an annotated git tag with those release notes
# 9. Pushes the tag and writes release notes to RELEASE_NOTES.md
# Usage: ./release.sh [major|minor|patch]
# Example: ./release.sh patch

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"
CURRENT_BRANCH="$(git rev-parse --abbrev-ref HEAD)"
CARGO_CMD="${CODETETHER_CARGO_CMD:-$SCRIPT_DIR/script/cargo-sccache.sh}"
VERIFY_CMD="${CODETETHER_RELEASE_VERIFY_CMD:-$CARGO_CMD test --quiet --lib --tests}"

# Read current version from Cargo.toml
CURRENT_VERSION="$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')"
if [ -z "$CURRENT_VERSION" ]; then
    echo "Error: Could not read version from Cargo.toml"
    exit 1
fi

# Parse version components — strip any pre-release suffix for bumping
BASE_VERSION="${CURRENT_VERSION%%-*}"
PRE_RELEASE="${CURRENT_VERSION#"$BASE_VERSION"}"  # e.g. "-alpha-8.2" or ""
PRE_RELEASE="${PRE_RELEASE#-}"                     # strip leading dash: "alpha-8.2" or ""
IFS='.' read -r MAJOR MINOR PATCH <<< "$BASE_VERSION"

# Parse pre-release sub-version if present (e.g. "alpha-8.2" → prefix="alpha" parts="8.2")
PRE_PREFIX=""
PRE_NUM=""
if [ -n "$PRE_RELEASE" ]; then
    # Split on last dot to get the sub-patch (e.g. "alpha-8.2" → "alpha-8" + "2")
    PRE_PREFIX="${PRE_RELEASE%.*}"   # "alpha-8"
    PRE_NUM="${PRE_RELEASE##*.}"     # "2"
    # If no dot, treat the whole thing as prefix
    if [ "$PRE_PREFIX" = "$PRE_RELEASE" ]; then
        PRE_NUM=""
    fi
fi

echo "Current version: $CURRENT_VERSION"

# Determine bump type — strip leading dashes for --flag style
BUMP="${1:-}"
BUMP="${BUMP#--}"
BUMP="${BUMP#-}"
if [ -z "$BUMP" ]; then
    echo ""
    if [ -n "$PRE_RELEASE" ]; then
        echo "Select bump type (pre-release detected):"
        if [ -n "$PRE_NUM" ]; then
            echo "  1) patch  → ${MAJOR}.${MINOR}.${PATCH}-${PRE_PREFIX}.$((PRE_NUM + 1))  (bump pre-release)"
        else
            echo "  1) patch  → ${MAJOR}.${MINOR}.${PATCH}-${PRE_RELEASE}.1  (bump pre-release)"
        fi
        echo "  2) minor  → ${MAJOR}.${MINOR}.$((PATCH + 1))  (release next patch)"
        echo "  3) major  → $((MAJOR + 1)).0.0"
    else
        echo "Select bump type:"
        echo "  1) patch  → ${MAJOR}.${MINOR}.$((PATCH + 1))"
        echo "  2) minor  → ${MAJOR}.$((MINOR + 1)).0"
        echo "  3) major  → $((MAJOR + 1)).0.0"
    fi
    echo ""
    read -rp "Choice [1/2/3]: " choice
    case "$choice" in
        1|patch)  BUMP="patch" ;;
        2|minor)  BUMP="minor" ;;
        3|major)  BUMP="major" ;;
        *)
            echo "Error: Invalid choice '$choice'"
            exit 1
            ;;
    esac
fi

# Calculate new version
if [ -n "$PRE_RELEASE" ]; then
    # Pre-release version: patch bumps pre-release, minor bumps patch (drops pre), major bumps major
    case "$BUMP" in
        patch)
            if [ -n "$PRE_NUM" ]; then
                new_version="${MAJOR}.${MINOR}.${PATCH}-${PRE_PREFIX}.$((PRE_NUM + 1))"
            else
                new_version="${MAJOR}.${MINOR}.${PATCH}-${PRE_RELEASE}.1"
            fi
            ;;
        minor) new_version="${MAJOR}.${MINOR}.$((PATCH + 1))" ;;
        major) new_version="$((MAJOR + 1)).0.0" ;;
        *)
            echo "Error: Invalid bump type '$BUMP'. Use patch, minor, or major."
            exit 1
            ;;
    esac
else
    # Stable version: standard semver bumps
    case "$BUMP" in
        patch) new_version="${MAJOR}.${MINOR}.$((PATCH + 1))" ;;
        minor) new_version="${MAJOR}.$((MINOR + 1)).0" ;;
        major) new_version="$((MAJOR + 1)).0.0" ;;
        *)
            echo "Error: Invalid bump type '$BUMP'. Use patch, minor, or major."
            exit 1
            ;;
    esac
fi

echo "==> Bumping $CURRENT_VERSION → $new_version"

# Step 1: Commit any pending changes using commit.sh
if ! git diff --quiet HEAD 2>/dev/null || ! git diff --cached --quiet 2>/dev/null; then
    echo "==> Committing pending changes via commit.sh..."
    "$SCRIPT_DIR/commit.sh"
fi

# Step 2: Get the last tag for changelog range
LAST_TAG="$(git describe --tags --abbrev=0 2>/dev/null || echo '')"
if [ -n "$LAST_TAG" ]; then
    COMMIT_RANGE="${LAST_TAG}..HEAD"
    echo "==> Generating release notes for ${LAST_TAG} → v${new_version}"
else
    COMMIT_RANGE="HEAD"
    echo "==> Generating release notes for v${new_version} (first release)"
fi

# Gather commit log and diffstat since last tag
COMMITS="$(git log --oneline --no-merges "$COMMIT_RANGE" 2>/dev/null || git log --oneline -20)"
DIFFSTAT="$(git diff --stat "$LAST_TAG" HEAD 2>/dev/null || git diff --stat HEAD~5 HEAD 2>/dev/null || echo 'N/A')"

# Step 3: Update version in Cargo.toml
sed -i "s|^version = \".*\"|version = \"${new_version}\"|" Cargo.toml

# Regenerate Cargo.lock
"$CARGO_CMD" generate-lockfile --quiet

# Step 4: Verify the release candidate
echo "==> Running release verification: $VERIFY_CMD"
if ! bash -lc "$VERIFY_CMD"; then
    echo "Error: release verification failed after version bump. Reverting."
    git restore Cargo.toml Cargo.lock
    exit 1
fi

# Step 5: Commit the version bump
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to $new_version"
git push origin "$CURRENT_BRANCH"

# Step 6: Generate AI release notes
RELEASE_MODEL="${CODETETHER_RELEASE_MODEL:-openai-codex/gpt-5.6-sol:high}"
echo "==> Generating release notes with codetether (model: $RELEASE_MODEL)..."
RAW_NOTES="$(RUST_LOG=error codetether run --model "$RELEASE_MODEL" "You are a release notes generator for a Rust CLI tool called CodeTether Agent.
Generate concise, professional release notes in markdown format for version v${new_version}.
Include sections: ## What's New, ## Bug Fixes (if any), ## Changes.
Base the notes on these commits and stats. Do NOT include a title heading — start directly with the sections.

Commits since ${LAST_TAG:-initial}:
${COMMITS}

Files changed:
${DIFFSTAT}" 2>/dev/null || echo "")"

# Strip ANSI codes and log noise
RELEASE_NOTES="$(echo "$RAW_NOTES" \
  | sed 's/\x1b\[[0-9;]*m//g' \
  | grep -vE '^\[Session:|^[0-9]{4}-[0-9]{2}-[0-9]{2}T|INFO |WARN |DEBUG |ERROR |^codetether::|Crash reporting' \
  || echo "")"

# Fallback if codetether returns empty
if [ -z "$RELEASE_NOTES" ]; then
    RELEASE_NOTES="Release v${new_version}

## Commits
${COMMITS}"
fi

# Write release notes to file
{
    echo "# v${new_version}"
    echo ""
    echo "$RELEASE_NOTES"
} > RELEASE_NOTES.md
git add RELEASE_NOTES.md
git commit -m "docs: release notes for v${new_version}" --allow-empty
git push origin "$CURRENT_BRANCH"

# Step 7: Create annotated tag with release notes and push
git tag -a "v$new_version" -m "$(echo "$RELEASE_NOTES")"
git push origin "v$new_version"

echo ""
echo "==> Release v$new_version created and pushed."
echo "==> Release notes written to RELEASE_NOTES.md"

# Step 8: Publish to crates.io
echo ""
echo "===> Publishing to crates.io..."

crate_version() {
    local crate="$1"
    cargo metadata --no-deps --format-version=1 \
        | jq -r --arg crate "$crate" '.packages[] | select(.name == $crate).version'
}

crate_version_exists() {
    local crate="$1"
    local version="$2"
    "$CARGO_CMD" search "$crate" --limit 1 \
        | grep -Eq "^$crate = \"$version\""
}

publish_crate() {
    local crate="$1"
    local wait_for_index="${2:-false}"
    local version
    local dry_run_output

    version="$(crate_version "$crate")"

    if crate_version_exists "$crate" "$version"; then
        echo "==> $crate v$version already exists on crates.io; skipping publish."
        return 0
    fi

    dry_run_output="$(mktemp)"
    if ! "$CARGO_CMD" publish --dry-run --no-verify -p "$crate" 2>&1 | tee "$dry_run_output"; then
        if grep -Eq "crate $crate@$version already exists" "$dry_run_output"; then
            echo "==> $crate v$version already exists on crates.io; skipping publish."
            rm -f "$dry_run_output"
            return 0
        fi

        rm -f "$dry_run_output"
        echo "Error: cargo publish --dry-run failed for $crate. Aborting publish."
        echo "Fix the errors above and re-run, or set CARGO_REGISTRY_TOKEN."
        return 1
    fi
    rm -f "$dry_run_output"

    if ! "$CARGO_CMD" publish --no-verify -p "$crate" 2>&1; then
        if "$CARGO_CMD" search "$crate" --limit 1 | grep -q "^$crate ="; then
            echo "==> $crate already exists on crates.io; continuing."
            return 0
        fi

        echo "Warning: cargo publish failed for $crate and it was not found on crates.io."
        echo "The crate may already exist at this version, or the API token is missing."
        echo "Set CARGO_REGISTRY_TOKEN or run: cargo login <token>"
        return 1
    fi

    echo "==> Published $crate to crates.io"

    if [ "$wait_for_index" = "true" ]; then
        wait_for_crate_index "$crate"
    fi
}

wait_for_crate_index() {
    local crate="$1"

    echo "==> Waiting for $crate to appear in crates.io index..."
    for _ in {1..30}; do
        if "$CARGO_CMD" search "$crate" --limit 1 | grep -q "^$crate ="; then
            echo "==> $crate is visible in crates.io index"
            return 0
        fi
        sleep 10
    done

    echo "Warning: $crate did not appear in crates.io index yet."
    echo "Retry the release publish step after crates.io finishes indexing."
    return 1
}

if ! publish_crate codetether-browser true || \
   ! publish_crate codetether-rlm true || \
   ! publish_crate codetether-agent false; then
    echo "Continuing with Jenkins release..."
fi

echo "==> Jenkins will build and create the GitHub release automatically."

echo ""
echo "===> Telling Jenkins to scan for new tag and create release..."
# Trigger Jenkins multibranch scan to detect the new tag and create the GitHub release
if [ -x "$SCRIPT_DIR/jenkinsfile.sh" ]; then
    if ! "$SCRIPT_DIR/jenkinsfile.sh" scan; then
        echo "Warning: Jenkins scan trigger failed."
        echo "Release artifacts/tag were already pushed."
        echo "Run '$SCRIPT_DIR/jenkinsfile.sh health' to diagnose, then retry '$SCRIPT_DIR/jenkinsfile.sh scan'."
    fi
else
    echo "Warning: $SCRIPT_DIR/jenkinsfile.sh not found/executable."
    echo "Release artifacts/tag were already pushed. Trigger Jenkins scan manually."
fi

echo "==> Done."

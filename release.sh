#!/bin/bash

# This script is used to create a tagged release of the codetether-agent. It performs the following steps:
# 1. Commits any pending changes via commit.sh (AI-generated commit message)
# 2. Validates version format
# 3. Updates the version number in Cargo.toml
# 4. Runs cargo check to verify the build
# 5. Commits the version bump and pushes to the main branch
# 6. Generates AI release notes from commits since the last tag
# 7. Creates an annotated git tag with those release notes
# 8. Pushes the tag and writes release notes to RELEASE_NOTES.md
# Usage: ./release.sh <new_version>
# Example: ./release.sh 1.1.6-alpha-8.2

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

if [ $# -ne 1 ]; then
    echo "Usage: $0 <new_version>"
    exit 1
fi

new_version="$1"

# Validate version format (semver with optional pre-release)
if ! [[ "$new_version" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9._-]+)?$ ]]; then
    echo "Error: Invalid version format '$new_version'"
    echo "Expected: MAJOR.MINOR.PATCH or MAJOR.MINOR.PATCH-prerelease"
    exit 1
fi

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
cargo generate-lockfile --quiet

# Step 4: Verify the project compiles
echo "==> Running cargo check..."
if ! cargo check --quiet 2>&1; then
    echo "Error: cargo check failed after version bump. Reverting."
    git checkout -- Cargo.toml Cargo.lock
    exit 1
fi

# Step 5: Commit the version bump
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to $new_version"
git push origin main

# Step 6: Generate AI release notes
echo "==> Generating release notes with codetether..."
RELEASE_NOTES="$(codetether run "You are a release notes generator for a Rust CLI tool called CodeTether Agent.
Generate concise, professional release notes in markdown format for version v${new_version}.
Include sections: ## What's New, ## Bug Fixes (if any), ## Changes.
Base the notes on these commits and stats. Do NOT include a title heading — start directly with the sections.

Commits since ${LAST_TAG:-initial}:
${COMMITS}

Files changed:
${DIFFSTAT}" 2>/dev/null || echo "Release v${new_version}")"

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
git push origin main

# Step 7: Create annotated tag with release notes and push
git tag -a "v$new_version" -m "$(echo "$RELEASE_NOTES")"
git push origin "v$new_version"

echo ""
echo "==> Release v$new_version created and pushed."
echo "==> Release notes written to RELEASE_NOTES.md"
echo "==> Jenkins will build and create the GitHub release automatically."
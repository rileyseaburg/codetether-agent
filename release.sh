#!/bin/bash

# This script is used to create a tagged release of the codetether-agent. It performs the following steps:
# 1. Validates version format
# 2. Updates the version number in Cargo.toml
# 3. Runs cargo check to verify the build
# 4. Commits the version bump and pushes to the main branch
# 5. Creates a git tag for the new version and pushes the tag to the remote repository.
# Usage: ./release.sh <new_version>
# Example: ./release.sh 1.1.6-alpha-8.2

set -euo pipefail

# cd to the directory containing this script
cd "$(dirname "$0")"

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

# Check for uncommitted changes
if ! git diff --quiet HEAD 2>/dev/null; then
    echo "Error: Working tree has uncommitted changes. Commit or stash them first."
    exit 1
fi

# Update version in Cargo.toml (use | as delimiter to avoid issues with version strings)
sed -i "s|^version = \".*\"|version = \"${new_version}\"|" Cargo.toml

# Regenerate Cargo.lock
cargo generate-lockfile --quiet

# Verify the project compiles
echo "Running cargo check..."
if ! cargo check --quiet 2>&1; then
    echo "Error: cargo check failed after version bump. Reverting."
    git checkout -- Cargo.toml Cargo.lock
    exit 1
fi

# Commit the version bump
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to $new_version"

# Push to main branch
git push origin main

# Create and push the tag
git tag "v$new_version"
git push origin "v$new_version"

echo "Release v$new_version created and pushed."
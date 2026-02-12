# v1.1.6-alpha-8.4

## What's New

- **DevOps Automation Scripts**: Added `commit.sh`, `release.sh`, and `jenkinsfile.sh` scripts for streamlined release workflows
- **Automatic Version Bumping**: `release.sh` now supports automatic version incrementing and pre-release tagging

## Bug Fixes

- Hardened `commit.sh` and `release.sh` scripts with improved error handling
- Updated Jenkinsfile to correctly generate release notes
- Added `jenkinsfile.sh` to `.gitignore` to prevent accidental commits

## Changes

- Improved output filtering in commit script
- Updated Jenkinsfile with enhanced release automation
- Version bumped to 1.1.6-alpha-8.4

**Stats**: 6 files changed, 331 insertions(+), 2 deletions(-)

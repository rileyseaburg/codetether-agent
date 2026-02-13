# v2.0.3

## What's New

- **Event Stream Module**: New `event_stream` module with JSONL audit logging and replay API for robust event tracking
- **S3/R2 Event Archival**: Configurable archival of audit events to S3/R2-compatible storage with relay checkpoint persistence for crash recovery
- **Replay API**: Ability to replay archived events for auditing and debugging workflows

## Bug Fixes

- **Memory Tool Performance**: Fixed a performance regression where the memory tool unnecessarily reloaded disk state on every execute call

## Changes

- Added 1,737 lines across 10 files with new event stream infrastructure
- Integrated event streaming into server, session, and TUI modules
- Added end-to-end tests for event stream functionality
- Session module now supports event stream integration for session lifecycle tracking

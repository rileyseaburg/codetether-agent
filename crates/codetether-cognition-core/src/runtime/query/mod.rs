//! Read-only cognition state projections and tool-registry attachment.
//! Runtime methods clone snapshots under existing locks or subscribe to events;
//! mutations are limited to replacing the optional tool registry.
mod beliefs;
mod events;
mod workspace;

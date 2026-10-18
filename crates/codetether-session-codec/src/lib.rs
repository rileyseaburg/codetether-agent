//! Borrowed session serialization and compact indexed-store headers.
//!
//! [HeaderView] supplies native scalar/metadata fields; [SnapshotView] adds
//! borrowed history. [serialize] preserves page fallback and runtime-aware
//! encoding. [encode_header] refreshes persisted metadata while [legacy_header]
//! preserves the imported metadata unchanged.
mod header;
mod header_view;
mod header_write;
mod normalize;
mod serialize;
mod snapshot;
mod snapshot_view;
pub use header::{decode_header, encode_header, legacy_header};
pub use header_view::HeaderView;
pub use serialize::serialize;
pub use snapshot_view::SnapshotView;

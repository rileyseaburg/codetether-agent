//! Recover images split across text-paste sidecars at submission time.
//! [recover_pasted_images] drains inline images before reassembling sidecars.
//! [State] borrows the sidecar list and shared image composer fields.
mod recover;
mod sidecar;
mod state;
pub use recover::recover_pasted_images;
pub use state::State;

//! Shared verifier model selection, verdicts and execution identity.
//!
//! Selection is process-wide while ObservationStore tracks the latest attempt.
//! Tickets capture actual provider resolution; verdicts come from final markers.
mod model;
pub mod observation;
mod selection;
mod self_review;
mod verdict;
pub use model::{VERIFIER_MODEL_ENV, resolve_verifier_model, select_verifier_model};
pub use selection::{
    VerifierSelection, selected_verifier_model, set_verifier_model, shared_selection,
};
pub use self_review::{is_self_review, warn_if_self_review};
pub use verdict::Verdict;

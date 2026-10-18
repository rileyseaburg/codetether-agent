//! Evidence classification, extraction and final-answer verification contracts.
//!
//! [claim::assess_claim] and [workflow::infer] classify text using stable marker
//! rules. [extract::from_text] returns serializable observations; [verification_gate::apply]
//! checks verification statements under the chosen [gate_mode::GateMode].
pub mod claim;
mod evidence_source;
pub mod extract;
mod extract_artifact;
mod extract_db;
mod extract_platform;
mod extract_runtime;
pub mod gate_mode;
pub mod level;
pub mod record;
pub mod recovery;
mod verification_claim;
pub mod verification_gate;
pub mod workflow;

/// Scope-completion status labels.
pub mod scope;

//! Shared approval reconciliation using the live native predicate.
codetether_approval_queue::define_reconciler!(crate::approval::live::is_pending);

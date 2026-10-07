//! Mock lease policy and request counters.
use std::sync::atomic::AtomicUsize;

pub(super) struct Counts {
    pub lookups: AtomicUsize,
    pub renewals: AtomicUsize,
    pub renewable: bool,
    pub ttl: u64,
    pub lookup_denied: bool,
    pub renew_status: u16,
}

impl Counts {
    pub fn new(renewable: bool, ttl: u64, lookup_denied: bool, renew_status: u16) -> Self {
        Self {
            lookups: AtomicUsize::new(0),
            renewals: AtomicUsize::new(0),
            renewable,
            ttl,
            lookup_denied,
            renew_status,
        }
    }
}

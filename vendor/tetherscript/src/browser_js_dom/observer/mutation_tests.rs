//! Mutation observer unit tests.

use super::mutation_observer::MutationObserverRegistry;
use super::mutation_types::{MutationObserverConfig, MutationRecord};
use std::{cell::RefCell, rc::Rc};

#[test]
fn mutation_delivery_filters_by_attribute_name() {
    let seen = Rc::new(RefCell::new(0));
    let out = seen.clone();
    let mut reg = MutationObserverRegistry::new();
    reg.observe(
        1,
        MutationObserverConfig {
            attributes: true,
            attribute_filter: vec!["class".into()],
            ..Default::default()
        },
        Box::new(move |r| {
            *out.borrow_mut() += r.len();
        }),
    );
    reg.queue_record(MutationRecord::attribute(1, "id", Some("a".into())), &[]);
    reg.queue_record(MutationRecord::attribute(1, "class", Some("a".into())), &[]);
    reg.deliver();
    assert_eq!(*seen.borrow(), 1);
}

#[test]
fn mutation_subtree_matches_ancestor() {
    let seen = Rc::new(RefCell::new(false));
    let out = seen.clone();
    let mut reg = MutationObserverRegistry::new();
    reg.observe(
        1,
        MutationObserverConfig {
            child_list: true,
            subtree: true,
            ..Default::default()
        },
        Box::new(move |_| *out.borrow_mut() = true),
    );
    reg.queue_record(MutationRecord::child_list(2, vec![3], vec![]), &[1]);
    reg.deliver();
    assert!(*seen.borrow());
}

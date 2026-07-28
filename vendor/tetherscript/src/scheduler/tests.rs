//! Unit tests for cooperative scheduler state transitions.

use crate::scheduler::Scheduler;
use crate::value::Value;

#[test]
fn spawn_queues_ready_tasks_in_order() {
    let mut scheduler = Scheduler::new();

    let first = scheduler.spawn();
    let second = scheduler.spawn();

    assert_eq!(scheduler.next_ready(), Some(first));
    assert_eq!(scheduler.next_ready(), Some(second));
    assert_eq!(scheduler.next_ready(), None);
}

#[test]
fn join_waits_until_all_targets_finish() {
    let mut scheduler = Scheduler::new();
    let waiter = scheduler.spawn();
    let first = scheduler.spawn();
    let second = scheduler.spawn();
    scheduler.next_ready();
    scheduler.next_ready();
    scheduler.next_ready();

    assert!(scheduler.join(waiter, &[first, second]).is_none());
    scheduler.finish(first, Value::Int(1));
    assert_eq!(scheduler.next_ready(), None);
    scheduler.finish(second, Value::Int(2));

    assert_eq!(scheduler.next_ready(), Some(waiter));
    let values = scheduler.join(waiter, &[first, second]).unwrap();
    assert!(matches!(values.as_slice(), [Value::Int(1), Value::Int(2)]));
}

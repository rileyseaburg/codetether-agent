//! Accepted input must survive handoff and races against completion.

use crate::Inboxes;
use std::sync::{Arc, Barrier};

#[test]
fn preserves_fifo_across_reopen_and_isolates_sessions() {
    let inboxes = Inboxes::default();
    inboxes.open("first");
    assert!(inboxes.push("first", 1).is_ok());
    inboxes.open("first");
    assert!(inboxes.push("first", 2).is_ok());
    inboxes.open("second");
    assert!(inboxes.push("second", 3).is_ok());
    assert_eq!(inboxes.take_or_close("first"), [1, 2]);
    assert!(inboxes.take_or_close("first").is_empty());
    assert_eq!(inboxes.push("first", 4), Err(4));
    assert_eq!(inboxes.discard("second"), [3]);
    assert_eq!(inboxes.push("second", 5), Err(5));
}

#[test]
fn concurrent_completion_either_delivers_or_rejects_input() {
    for _ in 0..64 {
        let inboxes = Arc::new(Inboxes::default());
        inboxes.open("session");
        let barrier = Arc::new(Barrier::new(2));
        let sender_inboxes = Arc::clone(&inboxes);
        let sender_barrier = Arc::clone(&barrier);
        let sender = std::thread::spawn(move || {
            sender_barrier.wait();
            sender_inboxes.push("session", 1)
        });
        barrier.wait();
        let delivered = inboxes.take_or_close("session");
        match sender.join().unwrap() {
            Ok(()) => assert_eq!(delivered, [1]),
            Err(input) => {
                assert_eq!(input, 1);
                assert!(delivered.is_empty());
            }
        }
        assert!(inboxes.take_or_close("session").is_empty());
        assert_eq!(inboxes.push("session", 2), Err(2));
    }
}

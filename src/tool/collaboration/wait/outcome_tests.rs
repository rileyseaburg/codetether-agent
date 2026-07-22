use super::{Activity, result};

#[test]
fn renders_mailbox_steering_and_timeout_outcomes() {
    let mailbox = result(Some(Activity::Mailbox));
    let steered = result(Some(Activity::Steered(1)));
    let timeout = result(None);
    assert!(mailbox.output.contains("Wait completed."));
    assert!(steered.output.contains("Wait interrupted by new input."));
    assert!(timeout.output.contains("\"timed_out\":true"));
}

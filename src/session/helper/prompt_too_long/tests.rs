use super::*;

#[test]
fn cascades_to_single_latest_message() {
    let err = anyhow::anyhow!("Your input exceeds the context window");

    assert_eq!(keep_last(&err, 1), Some(6));
    assert_eq!(keep_last(&err, 2), Some(3));
    assert_eq!(keep_last(&err, 3), Some(1));
    assert_eq!(keep_last(&err, 4), None);
}

#[test]
fn ignores_unrelated_errors() {
    let err = anyhow::anyhow!("connection reset");

    assert_eq!(keep_last(&err, 1), None);
}

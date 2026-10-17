use super::sign;

#[test]
fn creates_a_stable_server_compatible_proof() {
    let value = sign(
        "test-provenance-secret",
        "claim",
        "worker-1",
        "ctforgejo_author",
        "cttask_1",
        "1700000000",
    )
    .unwrap();
    assert_eq!(
        value,
        "a54f3fd301714efa72dd0be12b5558e9e5d552a8aefac3189955e567ed0814ce"
    );
}

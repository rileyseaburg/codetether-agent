use super::program;

#[test]
fn profile_is_c_bpf_instruction_stream() {
    assert!(!program(true).expect("supported architecture").is_empty());
}

#[test]
fn network_denied_profile_adds_socket_rules() {
    let denied = program(true).expect("supported architecture");
    let allowed = program(false).expect("supported architecture");
    assert!(denied.len() > allowed.len());
}
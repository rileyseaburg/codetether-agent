use super::program;

#[test]
fn profile_is_c_bpf_instruction_stream() {
    assert_eq!(program(true).len() % 8, 0);
}

#[test]
fn network_denied_profile_adds_socket_rules() {
    assert!(program(true).len() > program(false).len());
}

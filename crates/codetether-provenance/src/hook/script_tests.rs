use super::commit_msg_hook_script;

#[test]
fn hook_emits_the_forgejo_principal_contract() {
    let script = commit_msg_hook_script();
    for label in [
        "CodeTether-Forgejo-Host",
        "CodeTether-Forgejo-Login",
        "CodeTether-Agent-Slot",
    ] {
        assert!(script.contains(label));
    }
}

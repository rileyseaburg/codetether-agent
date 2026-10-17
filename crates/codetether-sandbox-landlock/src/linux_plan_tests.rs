//! linux plan tests for Landlock confinement.
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use codetether_sandbox_policy::SandboxPolicy;

#[test]
fn read_only_plan_keeps_temporary_storage_writable() {
    let rules =
        super::rules(&SandboxPolicy::default(), Path::new("/workspace"), 3).expect("sandbox rules");
    let temp = CString::new(std::env::temp_dir().as_os_str().as_bytes()).expect("temp path");
    let rule = rules
        .paths
        .iter()
        .find(|rule| rule.path == temp)
        .expect("temporary storage rule");
    assert!(rule.access & super::sys::REFER_ACCESS != 0);
    assert!(rule.access & super::sys::TRUNCATE_ACCESS != 0);
    assert_eq!(rule.access, rules.handled_access);
}

#[test]
fn plan_only_requests_rights_supported_by_the_kernel_abi() {
    let policy = SandboxPolicy::default();
    let abi_one = super::rules(&policy, Path::new("/workspace"), 1).expect("ABI 1 rules");
    assert_eq!(abi_one.handled_access & super::sys::REFER_ACCESS, 0);
    assert_eq!(abi_one.handled_access & super::sys::TRUNCATE_ACCESS, 0);

    let abi_two = super::rules(&policy, Path::new("/workspace"), 2).expect("ABI 2 rules");
    assert_ne!(abi_two.handled_access & super::sys::REFER_ACCESS, 0);
    assert_eq!(abi_two.handled_access & super::sys::TRUNCATE_ACCESS, 0);
}

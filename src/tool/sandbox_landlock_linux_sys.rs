#[repr(C)]
pub(crate) struct RulesetAttr {
    pub handled_access_fs: u64,
}

#[repr(C)]
pub(crate) struct PathBeneathAttr {
    pub allowed_access: u64,
    pub parent_fd: i32,
}

pub(crate) const VERSION_FLAG: u32 = 1;
pub(crate) const RULE_PATH_BENEATH: u32 = 1;
pub(crate) const READ_ACCESS: u64 = 1 | 4 | 8;
pub(crate) const WRITE_ACCESS: u64 = 2 | 16 | 32 | 64 | 128 | 256 | 512 | 1024 | 2048 | 4096;
pub(crate) const HANDLED_ACCESS: u64 = READ_ACCESS | WRITE_ACCESS;

pub(crate) fn abi_version() -> i64 {
    unsafe {
        libc::syscall(
            libc::SYS_landlock_create_ruleset,
            std::ptr::null::<libc::c_void>(),
            0usize,
            VERSION_FLAG,
        )
    }
}

pub(crate) unsafe fn create_ruleset(attr: &RulesetAttr) -> i32 {
    unsafe {
        libc::syscall(
            libc::SYS_landlock_create_ruleset,
            attr as *const _,
            std::mem::size_of::<RulesetAttr>(),
            0u32,
        ) as i32
    }
}

pub(crate) unsafe fn add_rule(fd: i32, rule: &PathBeneathAttr) -> i32 {
    unsafe {
        libc::syscall(
            libc::SYS_landlock_add_rule,
            fd,
            RULE_PATH_BENEATH,
            rule as *const _,
            0u32,
        ) as i32
    }
}

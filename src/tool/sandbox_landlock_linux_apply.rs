use super::plan::Rules;
use super::restrict::{close_with_error, restrict};
use super::sys::{self, PathBeneathAttr, RulesetAttr};

pub(crate) fn apply(cmd: &mut tokio::process::Command, rules: Option<Rules>) {
    let Some(rules) = rules else {
        return;
    };
    unsafe {
        cmd.pre_exec(move || install(&rules));
    }
}

fn install(rules: &Rules) -> std::io::Result<()> {
    let attr = RulesetAttr {
        handled_access_fs: sys::HANDLED_ACCESS,
    };
    let fd = unsafe { sys::create_ruleset(&attr) };
    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }
    add_rules(fd, rules).and_then(|_| restrict(fd))
}

fn add_rules(fd: i32, rules: &Rules) -> std::io::Result<()> {
    for rule in &rules.paths {
        let parent_fd = unsafe { libc::open(rule.path.as_ptr(), libc::O_PATH | libc::O_CLOEXEC) };
        if parent_fd < 0 {
            return close_with_error(fd);
        }
        let attr = PathBeneathAttr {
            allowed_access: rule.access,
            parent_fd,
        };
        let added = unsafe { sys::add_rule(fd, &attr) };
        unsafe { libc::close(parent_fd) };
        if added != 0 {
            return close_with_error(fd);
        }
    }
    Ok(())
}

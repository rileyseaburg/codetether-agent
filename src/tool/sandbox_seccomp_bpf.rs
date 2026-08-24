#[path = "sandbox_seccomp_syscalls.rs"]
mod syscalls;

const LD_W_ABS: u16 = 0x20;
const JMP_JEQ_K: u16 = 0x15;
const RET_K: u16 = 0x06;
const SECCOMP_ALLOW: u32 = 0x7fff0000;
const SECCOMP_ERRNO: u32 = 0x00050000 | libc::EPERM as u32;

pub(super) fn program(deny_network: bool) -> Option<Vec<u64>> {
    let audit_arch = syscalls::audit_arch()?;
    let mut out = Vec::new();
    emit(&mut out, LD_W_ABS, 0, 0, 4);
    emit(&mut out, JMP_JEQ_K, 1, 0, audit_arch);
    emit(&mut out, RET_K, 0, 0, SECCOMP_ERRNO);
    emit(&mut out, LD_W_ABS, 0, 0, 0);
    for nr in syscalls::denied(deny_network) {
        deny(&mut out, nr);
    }
    emit(&mut out, RET_K, 0, 0, SECCOMP_ALLOW);
    Some(out)
}

fn deny(out: &mut Vec<u64>, syscall: u32) {
    emit(out, JMP_JEQ_K, 0, 1, syscall);
    emit(out, RET_K, 0, 0, SECCOMP_ERRNO);
}

fn emit(out: &mut Vec<u64>, code: u16, jt: u8, jf: u8, k: u32) {
    let mut bytes = [0u8; 8];
    bytes[..2].copy_from_slice(&code.to_ne_bytes());
    bytes[2] = jt;
    bytes[3] = jf;
    bytes[4..].copy_from_slice(&k.to_ne_bytes());
    out.push(u64::from_ne_bytes(bytes));
}

#[cfg(test)]
#[path = "sandbox_seccomp_bpf_tests.rs"]
mod tests;
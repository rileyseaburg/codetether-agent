const LD_W_ABS: u16 = 0x20;
const JMP_JEQ_K: u16 = 0x15;
const RET_K: u16 = 0x06;
const SECCOMP_ALLOW: u32 = 0x7fff0000;
const SECCOMP_ERRNO: u32 = 0x00050000 | libc::EPERM as u32;
const AUDIT_ARCH_X86_64: u32 = 0xc000003e;

pub(super) fn program(deny_network: bool) -> Vec<u8> {
    let mut out = Vec::new();
    emit(&mut out, LD_W_ABS, 0, 0, 4);
    emit(&mut out, JMP_JEQ_K, 1, 0, AUDIT_ARCH_X86_64);
    emit(&mut out, RET_K, 0, 0, SECCOMP_ERRNO);
    emit(&mut out, LD_W_ABS, 0, 0, 0);
    for nr in denied_syscalls(deny_network) {
        deny(&mut out, nr);
    }
    emit(&mut out, RET_K, 0, 0, SECCOMP_ALLOW);
    out
}

fn denied_syscalls(deny_network: bool) -> Vec<u32> {
    let mut out = vec![
        101, 175, 176, 246, 248, 249, 250, 298, 304, 310, 311, 313, 320, 321, 323,
    ];
    if deny_network {
        out.extend([
            41, 42, 43, 44, 45, 46, 47, 49, 50, 53, 54, 55, 288, 299, 307,
        ]);
    }
    out
}

fn deny(out: &mut Vec<u8>, syscall: u32) {
    emit(out, JMP_JEQ_K, 0, 1, syscall);
    emit(out, RET_K, 0, 0, SECCOMP_ERRNO);
}

fn emit(out: &mut Vec<u8>, code: u16, jt: u8, jf: u8, k: u32) {
    out.extend(code.to_ne_bytes());
    out.extend([jt, jf]);
    out.extend(k.to_ne_bytes());
}

#[cfg(test)]
#[path = "sandbox_seccomp_bpf_tests.rs"]
mod tests;

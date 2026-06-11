pub(super) fn push_pair(out: &mut Vec<String>, op: &str, value: &str) {
    out.extend([op.to_string(), value.to_string()]);
}

pub(super) fn push_triple(out: &mut Vec<String>, op: &str, src: &str, dst: &str) {
    out.extend([op.to_string(), src.to_string(), dst.to_string()]);
}

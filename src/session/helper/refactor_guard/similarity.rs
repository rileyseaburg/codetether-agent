pub fn ratio(old: &str, new: &str) -> f32 {
    let old = super::lines::code_line_set(old);
    let new = super::lines::code_line_set(new);
    let base = old.len().min(new.len());
    if base == 0 {
        return 0.0;
    }
    let common = old.intersection(&new).count();
    common as f32 / base as f32
}

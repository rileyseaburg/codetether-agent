//! Runtime geometry for the verified text-only Qwen3.5 27B checkpoint.
pub(super) struct Config {
    pub layers: usize,
    pub hidden: usize,
    pub heads: usize,
    pub kv_heads: usize,
    pub head: usize,
    pub keys: usize,
    pub values: usize,
    pub state: usize,
    pub conv: usize,
    pub rotary: usize,
    pub eps: f64,
    pub base: f64,
    pub context: usize,
}

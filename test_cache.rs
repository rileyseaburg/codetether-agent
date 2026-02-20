use candle_transformers::models::quantized_gemma3;
fn main() {
    let mut w: quantized_gemma3::ModelWeights = unsafe { std::mem::zeroed() };
    w.clear_kv_cache();
}

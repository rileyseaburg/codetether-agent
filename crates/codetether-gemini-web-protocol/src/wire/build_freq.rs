//! build freq.
use crate::*;
impl GeminiWebProtocol {
    /// Build the `f.req` JSON payload — a two-element outer array whose
    /// second slot is a JSON-encoded 69-element inner array.
    ///
    /// Index layout matches the Python GemChat reference implementation.
    pub fn build_freq(prompt: &str) -> String {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let mut inner: Vec<Value> = vec![Value::Null; 69];
        // [0]  prompt tuple — single array, NOT double-wrapped
        inner[0] = json!([prompt, 0, null, null, null, null, 0]);
        // [1]  language
        inner[1] = json!(["en"]);
        // [2]  conversation continuation ids (empty strings for new convo)
        inner[2] = json!(["", "", "", null, null, null, null, null, null, ""]);
        // [3]-[5] null
        inner[6] = json!([1]);
        inner[7] = json!(1);
        // [8]-[9] null
        inner[10] = json!(1);
        inner[11] = json!(0);
        // [12] null
        // [13]-[16] null
        inner[17] = json!([[0]]);
        inner[18] = json!(0);
        // [19]-[26] null
        inner[27] = json!(1);
        // [28]-[29] null
        inner[30] = json!([4]);
        // [31]-[52] null
        inner[53] = json!(0);
        // [54]-[58] null
        inner[59] = json!("CD1035A5-0E0E-4B68-B744-23C2D8960DF5");
        // [60] null
        inner[61] = json!([]);
        // [62]-[65] null
        inner[66] = json!([ts, 0]);
        // [67] null
        inner[68] = json!(2);

        debug_assert_eq!(
            inner.len(),
            69,
            "f.req inner list must be exactly 69 elements"
        );

        let inner_json = serde_json::to_string(&inner).unwrap_or_default();
        serde_json::to_string(&json!([null, inner_json])).unwrap_or_default()
    }
}

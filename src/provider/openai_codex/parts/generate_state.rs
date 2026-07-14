impl OpenAiCodexProvider {
    fn generate_state() -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let random: [u8; 8] = {
            let ptr = Box::into_raw(Box::new(timestamp)) as usize;
            let bytes = ptr.to_le_bytes();
            let mut arr = [0u8; 8];
            arr.copy_from_slice(&bytes);
            arr
        };
        format!("{:016x}{:016x}", timestamp, u64::from_le_bytes(random))
    }
}

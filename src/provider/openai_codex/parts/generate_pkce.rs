impl OpenAiCodexProvider {
    fn generate_pkce() -> PkcePair {
        let random_bytes: [u8; 32] = {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0);

            let mut bytes = [0u8; 32];
            let ts_bytes = timestamp.to_le_bytes();
            let tid = std::thread::current().id();
            let tid_repr = format!("{:?}", tid);
            let tid_hash = Sha256::digest(tid_repr.as_bytes());

            bytes[0..8].copy_from_slice(&ts_bytes[0..8]);
            bytes[8..24].copy_from_slice(&tid_hash[0..16]);
            bytes[24..].copy_from_slice(&Sha256::digest(ts_bytes)[0..8]);
            bytes
        };
        let verifier = URL_SAFE_NO_PAD.encode(random_bytes);

        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let challenge_bytes = hasher.finalize();
        let challenge = URL_SAFE_NO_PAD.encode(challenge_bytes);

        PkcePair {
            verifier,
            challenge,
        }
    }
}

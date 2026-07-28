use super::super::*;

#[test]
fn crypto_random_uuid_is_seeded_v4_and_consumes_rng_stream() {
    let result = eval_with_dom(
        "<main></main>",
        "crypto.setRandomSeed(7); let id=crypto.randomUUID();\
         let after=Uint8Array(4); crypto.getRandomValues(after);\
         crypto.setRandomSeed(7); let raw=Uint8Array(20); crypto.getRandomValues(raw);\
         id+'|'+after.join('-')+'|'+raw.join('-');",
    )
    .unwrap();
    let JsValue::String(text) = result.value else {
        panic!("expected randomUUID probe string");
    };
    let parts: Vec<&str> = text.split('|').collect();
    assert_eq!(parts.len(), 3);
    assert_uuid(parts[0]);
    let raw = parse_bytes(parts[2]);
    assert_eq!(parse_bytes(parts[1]), raw[16..20]);
    let mut uuid_bytes = raw[..16].to_vec();
    uuid_bytes[6] = (uuid_bytes[6] & 15) | 64;
    uuid_bytes[8] = (uuid_bytes[8] & 63) | 128;
    assert_eq!(parts[0], uuid(&uuid_bytes));
}

fn parse_bytes(text: &str) -> Vec<u8> {
    text.split('-').map(|byte| byte.parse().unwrap()).collect()
}

fn assert_uuid(id: &str) {
    assert_eq!(id.len(), 36);
    for index in [8, 13, 18, 23] {
        assert_eq!(id.as_bytes()[index], b'-');
    }
    assert_eq!(id.as_bytes()[14], b'4');
    assert!(matches!(id.as_bytes()[19], b'8' | b'9' | b'a' | b'b'));
    assert!(id.bytes().all(|b| b == b'-' || b.is_ascii_hexdigit()));
}

fn uuid(bytes: &[u8]) -> String {
    let mut out = String::new();
    for (index, byte) in bytes.iter().enumerate() {
        if [4, 6, 8, 10].contains(&index) {
            out.push('-');
        }
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

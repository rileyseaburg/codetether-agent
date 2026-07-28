use super::super::*;

#[test]
fn crypto_random_values_are_seedable_and_mutate_uint8_array() {
    let result = eval_with_dom(
        "<main></main>",
        "crypto.setRandomSeed(7); let a=Uint8Array(4); crypto.getRandomValues(a); let first=a.join('-'); crypto.setRandomSeed(7); let b=Uint8Array(4); crypto.getRandomValues(b); first == b.join('-') && first != '0-0-0-0' && a.length == 4;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::Bool(true));
}

#[test]
fn crypto_subtle_digest_sha256_returns_byte_array_promise() {
    let result = eval_with_dom(
        "<main></main>",
        "let out=''; crypto.subtle.digest('SHA-256', TextEncoder().encode('abc')).then(function(hash){ out=hash.length + ':' + hash[0] + ':' + hash[1] + ':' + hash[2] + ':' + hash[3]; }); out;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("32:186:120:22:191".into()));
}

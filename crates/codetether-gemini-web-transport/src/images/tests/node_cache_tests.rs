use crate::images::{cache_io, cookies::now, netscape, node_cache};
use serde_json::json;
use sha2::{Digest, Sha256};

#[test]
fn node_cache_migrates_only_private_matching_valid_credentials() {
    let root = tempfile::tempdir().unwrap();
    let directory = root.path().join("cache");
    let source = ".google.com\tTRUE\t/\tTRUE\t0\t__Secure-1PSID\tsynthetic\n";
    let jar = netscape::parse(source).unwrap();
    let key: String = Sha256::digest(b"__Secure-1PSID=synthetic")
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let instant = now();
    let valid = json!({"updated": instant * 1000, "cookies": [{
        "name": "__Secure-1PSIDTS", "value": "renewed", "expires": (instant + 3600) * 1000
    }]});
    let write = |value: &serde_json::Value| {
        cache_io::write(&directory, &key, &serde_json::to_vec(value).unwrap()).unwrap();
    };
    write(&valid);
    let (updated, cookies) = node_cache::load(&directory, &jar).unwrap();
    assert_eq!(updated, instant);
    assert_eq!(cookies[0].expires, instant + 3600);
    let other = netscape::parse(&source.replace("synthetic", "other")).unwrap();
    assert!(node_cache::load(&directory, &other).is_none());
    for (field, value) in [
        ("expires", json!(1000)),
        ("name", json!("SID")),
        ("value", json!("secret\r\ninjected")),
        ("value", json!("")),
    ] {
        let mut bad = valid.clone();
        bad["cookies"][0][field] = value;
        write(&bad);
        assert!(node_cache::load(&directory, &jar).is_none());
    }
    let mut future = valid.clone();
    future["updated"] = json!((instant + 3600) * 1000);
    write(&future);
    assert!(node_cache::load(&directory, &jar).is_none());
    write(&valid);
    super::node_cache_permissions::verify(&directory, &key, &jar);
}

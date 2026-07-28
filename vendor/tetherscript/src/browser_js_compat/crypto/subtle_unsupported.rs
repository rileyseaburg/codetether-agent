use super::*;

#[path = "subtle_reject.rs"]
mod reject;

const METHODS: [&str; 10] = [
    "decrypt",
    "deriveBits",
    "deriveKey",
    "encrypt",
    "exportKey",
    "generateKey",
    "importKey",
    "sign",
    "unwrapKey",
    "verify",
];

pub(super) fn install(subtle: &mut HashMap<String, JsValue>) {
    for method in METHODS {
        let name = format!("crypto.subtle.{method}");
        subtle.insert(
            method.into(),
            native(&name, None, move |_| Ok(reject::promise(method))),
        );
    }
}

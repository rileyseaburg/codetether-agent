use super::*;

#[path = "crypto_digest.rs"]
mod digest;
#[path = "crypto/random_uuid.rs"]
mod random_uuid;
#[path = "crypto/seed.rs"]
mod seed;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    let seed = Rc::new(RefCell::new(seed::DEFAULT));
    let mut crypto = HashMap::new();
    get_random_values(&mut crypto, seed.clone());
    set_random_seed(&mut crypto, seed.clone());
    random_uuid::install(&mut crypto, seed);
    crypto.insert("subtle".into(), digest::subtle_object());
    window.insert(
        "crypto".into(),
        JsValue::Object(Rc::new(RefCell::new(crypto))),
    );
}

fn get_random_values(crypto: &mut HashMap<String, JsValue>, seed: Rc<RefCell<u64>>) {
    crypto.insert(
        "getRandomValues".into(),
        native("crypto.getRandomValues", Some(1), move |args| {
            let target = args.first().cloned().unwrap_or(JsValue::Undefined);
            let mut state = seed.borrow_mut();
            bytes::fill_array(&target, || seed::next_byte(&mut state))?;
            Ok(target)
        }),
    );
}

fn set_random_seed(crypto: &mut HashMap<String, JsValue>, seed: Rc<RefCell<u64>>) {
    crypto.insert(
        "setRandomSeed".into(),
        native("crypto.setRandomSeed", Some(1), move |args| {
            *seed.borrow_mut() = seed::value(args.first());
            Ok(JsValue::Undefined)
        }),
    );
}

mod common;
use codetether_companion_core::{Error, Registry};

#[test]
fn model_and_prompt_validation_matches_relay_boundaries() {
    for model in ["p/m", "p/m/n", "p//", "p/m/"] {
        let mut input = common::input();
        input.model = model.into();
        assert!(Registry::default().create(input, common::NOW).is_ok());
    }
    for model in ["p", "/m", "p/", "p m/m", "p/m?", "p/m\n"] {
        let mut input = common::input();
        input.model = model.into();
        assert_eq!(
            Registry::default().create(input, common::NOW).err(),
            Some(Error::Input)
        );
    }
    for (prompt, accepted) in [
        ("😀".repeat(1000), true),
        ("😀".repeat(1001), false),
        ("\u{feff}\u{00a0}".into(), false),
        ("\u{0085}".into(), true),
    ] {
        let mut input = common::input();
        input.prompt = prompt;
        assert_eq!(
            Registry::default().create(input, common::NOW).is_ok(),
            accepted
        );
    }
}

#[test]
fn model_length_is_bounded() {
    for (length, accepted) in [(198, true), (199, false)] {
        let mut input = common::input();
        input.model = format!("p/{}", "m".repeat(length));
        assert_eq!(
            Registry::default().create(input, common::NOW).is_ok(),
            accepted
        );
    }
}

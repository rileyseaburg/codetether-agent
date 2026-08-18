//! Static Z.AI model catalog entries.

pub type Entry = (
    &'static str,
    &'static str,
    usize,
    usize,
    Option<f64>,
    Option<f64>,
);

pub const KNOWN_MODELS: &[Entry] = &[
    ("glm-5.3", "GLM-5.3", 1_000_000, 128_000, None, None),
    ("glm-5.2", "GLM-5.2", 1_000_000, 128_000, None, None),
    ("glm-5.1", "GLM-5.1", 200_000, 128_000, None, None),
    ("glm-5", "GLM-5", 200_000, 128_000, None, None),
    ("glm-4.7", "GLM-4.7", 128_000, 128_000, None, None),
    (
        "glm-4.7-flash",
        "GLM-4.7 Flash",
        128_000,
        128_000,
        None,
        None,
    ),
    ("glm-4.6", "GLM-4.6", 128_000, 128_000, None, None),
    ("glm-4.5", "GLM-4.5", 128_000, 96_000, None, None),
    (
        "glm-5-turbo",
        "GLM-5 Turbo",
        200_000,
        128_000,
        Some(0.96),
        Some(3.20),
    ),
    ("pony-alpha-2", "Pony Alpha 2", 128_000, 16_384, None, None),
];

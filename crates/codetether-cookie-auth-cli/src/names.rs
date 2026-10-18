//! Cookie import names responsibilities.

pub(super) fn preferred_cookie_names(provider_id: &str) -> &'static [&'static str] {
    match provider_id {
        "nextdoor-web" => &[
            "ndbr_at",
            "ndbr_idt",
            "ndbr_adt",
            "csrftoken",
            "ndp_session_id",
            "WE",
            "WE3P",
            "DAID",
        ],
        "gemini-web" => &[
            "__Secure-1PSID",
            "__Secure-1PSIDTS",
            "__Secure-1PSIDCC",
            "SID",
            "HSID",
            "SSID",
            "APISID",
            "SAPISID",
        ],
        _ => &[],
    }
}

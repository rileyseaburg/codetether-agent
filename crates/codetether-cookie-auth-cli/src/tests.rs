//! Cookie parsing and provider-filtering regressions.
use crate::{parse::parse_netscape_cookie_line, select::select_cookie_rows};

#[test]
fn parses_netscape_cookie_with_httponly_prefix() {
    let line = "#HttpOnly_.nextdoor.com\tTRUE\t/\tTRUE\t1803495701\tndbr_at\ttoken123";
    let parsed = parse_netscape_cookie_line(line).expect("expected cookie parse");
    assert_eq!(parsed.domain, ".nextdoor.com");
    assert!(parsed.http_only);
    assert_eq!(parsed.name, "ndbr_at");
}

#[test]
fn nextdoor_filter_keeps_auth_cookies_only() {
    let rows = vec![
        parse_netscape_cookie_line(".nextdoor.com\tTRUE\t/\tTRUE\t4803495701\tndbr_at\tauth-token")
            .expect("auth cookie"),
        parse_netscape_cookie_line(".nextdoor.com\tTRUE\t/\tFALSE\t4803495701\t_ga\ttracking")
            .expect("tracking cookie"),
    ];
    let (selected, dropped_expired, dropped_non_auth) =
        select_cookie_rows(&rows, "nextdoor-web", false);
    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].name, "ndbr_at");
    assert_eq!(dropped_expired, 0);
    assert_eq!(dropped_non_auth, 1);
}

//! Offline parser and rendering tests using captured page shapes.
use crate::{Backend, SearchFailure, attempt::Attempt, bing, duckduckgo, report};

const BING_OK: &str = r#"<ol id="b_results"><li class="b_algo" data-id><h2><a target="_blank" href="https://www.bing.com/ck/a?!&amp;&amp;p=abc&amp;u=a1aHR0cHM6Ly9hc2NlbmRhdGVsZXZvbi5jb20v&amp;ntb=1" h="ID=SERP,1"><strong>Lavon apartments</strong> for rent | Ascend</a></h2><div class="b_caption"><p class="b_lineclamp2">Ascend at Elevon in Lavon, TX &amp; more</p></div></li></ol>"#;

#[test]
fn bing_unwraps_tracking_links_and_strips_markup() {
    let results = bing::parse(200, BING_OK, 5).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].url, "https://ascendatelevon.com/");
    assert_eq!(results[0].title, "Lavon apartments for rent | Ascend");
    assert_eq!(results[0].snippet, "Ascend at Elevon in Lavon, TX & more");
}

#[test]
fn bing_without_result_list_is_never_zero_results() {
    let challenge = bing::parse(200, "<html>captcha</html>", 5);
    assert_eq!(challenge, Err(SearchFailure::Challenge { status: 200 }));
    let malformed = bing::parse(200, "<html>changed layout</html>", 5);
    assert!(matches!(malformed, Err(SearchFailure::Malformed(_))));
    assert_eq!(
        bing::parse(429, "", 5),
        Err(SearchFailure::RateLimited { status: 429 })
    );
    assert!(
        bing::parse(200, r#"<ol id="b_results"></ol>"#, 5)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn duckduckgo_challenge_page_is_a_failure() {
    let page = r#"<div class="anomaly-modal__title">bots</div><form id="challenge-form">"#;
    assert_eq!(
        duckduckgo::parse(202, page, 5),
        Err(SearchFailure::Challenge { status: 202 })
    );
    assert!(matches!(
        duckduckgo::parse(200, "<html></html>", 5),
        Err(SearchFailure::Malformed(_))
    ));
}

#[test]
fn failed_attempts_render_as_error_with_evidence() {
    let attempts = [Attempt {
        backend: Backend::DuckDuckGo,
        status: Some(202),
        request_id: None,
        outcome: Err(SearchFailure::Challenge { status: 202 }),
    }];
    let result = report::render("q", &attempts);
    assert!(!result.success);
    assert!(!result.output.contains("No results found"));
    assert!(result.output.contains("WEBSEARCH_PROVIDER_CHALLENGE"));
    assert_eq!(
        result.metadata["search_evidence"]["attempts"][0]["http_status"],
        202
    );
}

#[test]
fn success_keeps_url_lines_and_names_backend() {
    let attempts = [Attempt {
        backend: Backend::Bing,
        status: Some(200),
        request_id: Some("evt".into()),
        outcome: bing::parse(200, BING_OK, 5),
    }];
    let result = report::render("q", &attempts);
    assert!(result.success);
    assert!(
        result
            .output
            .contains("\n   URL: https://ascendatelevon.com/\n")
    );
    let evidence = &result.metadata["search_evidence"];
    assert_eq!(evidence["backend"], "bing");
    assert_eq!(evidence["sources"][0], "https://ascendatelevon.com/");
    let line = result.output.lines().last().unwrap();
    assert!(line.starts_with(crate::EVIDENCE_PREFIX));
}

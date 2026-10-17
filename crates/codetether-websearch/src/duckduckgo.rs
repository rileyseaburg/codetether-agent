//! duckduckgo for web search.
use super::*;
impl WebSearchTool {
    pub(crate) async fn search_ddg(
        &self,
        query: &str,
        max_results: usize,
    ) -> Result<Vec<SearchResult>> {
        // DuckDuckGo HTML search (no API key required)
        let url = format!(
            "https://html.duckduckgo.com/html/?q={}",
            urlencoding::encode(query)
        );
        let resp = self.client.get(&url).send().await?;
        let html = resp.text().await?;

        let mut results = Vec::new();
        // Parse result links from DDG HTML
        let link_re =
            regex::Regex::new(r#"<a[^>]+class="result__a"[^>]+href="([^"]+)"[^>]*>([^<]+)</a>"#)?;
        let snippet_re = regex::Regex::new(r#"<a[^>]+class="result__snippet"[^>]*>([^<]+)</a>"#)?;

        let links: Vec<_> = link_re.captures_iter(&html).collect();
        let snippets: Vec<_> = snippet_re.captures_iter(&html).collect();

        for (i, cap) in links.iter().take(max_results).enumerate() {
            let url = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let title = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let snippet = snippets
                .get(i)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str())
                .unwrap_or("");

            // DDG wraps URLs, extract actual URL
            let actual_url = if url.contains("uddg=") {
                url.split("uddg=")
                    .nth(1)
                    .and_then(|s| urlencoding::decode(s.split('&').next().unwrap_or("")).ok())
                    .map(|s| s.into_owned())
                    .unwrap_or_else(|| url.to_string())
            } else {
                url.to_string()
            };

            results.push(SearchResult {
                title: html_escape::decode_html_entities(title).to_string(),
                url: actual_url,
                snippet: html_escape::decode_html_entities(snippet).to_string(),
            });
        }
        Ok(results)
    }
}

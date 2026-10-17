//! Extract the original JSON candidate from fenced or surrounding text.
pub(super) fn extract(response: &str) -> &str {
    // Try to extract JSON from the response
    let json_str = if let Some(start) = response.find("```json") {
        let start = start + 7;
        if let Some(end) = response[start..].find("```") {
            &response[start..start + end]
        } else {
            response
        }
    } else if let Some(start) = response.find('{') {
        if let Some(end) = response.rfind('}') {
            &response[start..=end]
        } else {
            response
        }
    } else {
        response
    };
    json_str
}

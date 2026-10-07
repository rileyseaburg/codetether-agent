import Foundation

/// Restrict exported HTML to inert formatting before UIKit imports it as rich text.
/// Keep only safe links and inert tags; copying cannot fetch resources or execute HTML.
enum RichMessageHTML {
    private static let tags: Set<String> = [
        "p", "div", "span", "h1", "h2", "h3", "h4", "h5", "h6", "strong", "b",
        "em", "i", "s", "del", "code", "pre", "blockquote", "ul", "ol", "li", "br",
        "hr", "table", "thead", "tbody", "tr", "th", "td", "a"
    ]
    static func document(_ html: String) throws -> String {
        let regex = try NSRegularExpression(pattern: #"<\s*(/?)\s*([a-zA-Z][a-zA-Z0-9]*)\b[^>]*>"#)
        let source = html as NSString
        var result = "", offset = 0
        for match in regex.matches(in: html, range: NSRange(location: 0, length: source.length)) {
            result += escapeAngles(source.substring(with: NSRange(location: offset, length: match.range.location - offset)))
            let name = source.substring(with: match.range(at: 2)).lowercased()
            if tags.contains(name) {
                let slash = source.substring(with: match.range(at: 1))
                let link = name == "a" && slash.isEmpty
                    ? SafeRichLink.attribute(source.substring(with: match.range)) : ""
                result += "<\(slash)\(name)\(link)>"
            }
            offset = NSMaxRange(match.range)
        }
        result += escapeAngles(source.substring(from: offset))
        return """
        <html><head><meta charset="utf-8"><style>
        body { font-family: -apple-system; font-size: 16px; }
        pre, code { font-family: Menlo, monospace; } th { font-weight: bold; }
        </style></head><body>\(result)</body></html>
        """
    }
    private static func escapeAngles(_ text: String) -> String {
        text.replacingOccurrences(of: "<", with: "&lt;").replacingOccurrences(of: ">", with: "&gt;")
    }
}
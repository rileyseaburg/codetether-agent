import Foundation

/// Preserve only explicit web/mail links while discarding all other HTML attributes.
enum SafeRichLink {
    static func attribute(_ tag: String) -> String {
        guard let regex = try? NSRegularExpression(pattern: #"(?i)\bhref\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s>]+))"#),
              let match = regex.firstMatch(in: tag, range: NSRange(tag.startIndex..., in: tag)) else { return "" }
        let source = tag as NSString
        for index in 1...3 where match.range(at: index).location != NSNotFound {
            let value = source.substring(with: match.range(at: index))
                .replacingOccurrences(of: "&amp;", with: "&")
            guard let url = URL(string: value),
                  ["https", "http", "mailto"].contains(url.scheme?.lowercased() ?? "") else { return "" }
            let escaped = value.replacingOccurrences(of: "&", with: "&amp;")
                .replacingOccurrences(of: "\"", with: "&quot;")
                .replacingOccurrences(of: "<", with: "&lt;")
                .replacingOccurrences(of: ">", with: "&gt;")
            return " href=\"\(escaped)\""
        }
        return ""
    }
}

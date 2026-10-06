import Foundation

enum ImageReferencePaths {
    private static let pattern = #"(/home/[^\s"\)]+\.(?:png|jpg|jpeg))"#
    private static let expression = try! NSRegularExpression(pattern: pattern)
    static func extract(_ text: String) -> [String] {
        let value = text as NSString
        return expression.matches(in: text, range: NSRange(location: 0, length: value.length))
            .map { value.substring(with: $0.range(at: 1)) }
    }
}

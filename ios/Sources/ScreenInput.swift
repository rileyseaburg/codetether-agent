import Foundation

/// Matches the relay's UTF-16 limits, including non-BMP characters such as emoji.
enum ScreenInput {
    static func text(_ value: String) -> Bool {
        !value.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty
            && value.utf16.count <= 2000
    }
    static func model(_ value: String) -> Bool {
        value.utf16.count <= 200
            && value.range(of: #"^[a-zA-Z0-9_.:-]+/[a-zA-Z0-9_./:-]+$"#,
                           options: .regularExpression) == (value.startIndex..<value.endIndex)
    }
}

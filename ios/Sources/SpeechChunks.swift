import Foundation

enum SpeechChunks {
    static func split(_ text: String) -> [String] {
        let cleaned = text.replacingOccurrences(of: "(?s)```.*?```", with: " Code block. ", options: .regularExpression)
            .replacingOccurrences(of: "[*#`_]", with: "", options: .regularExpression)
        var chunks: [String] = []
        var current = ""
        for word in cleaned.split(whereSeparator: { $0.isWhitespace }) {
            let addition = current.isEmpty ? String(word) : " " + word
            if current.utf16.count + addition.utf16.count <= 380 { current += addition; continue }
            if !current.isEmpty { chunks.append(current); current = "" }
            for character in word {
                if current.utf16.count + String(character).utf16.count > 380 {
                    chunks.append(current); current = ""
                }
                current.append(character)
            }
        }
        if !current.isEmpty { chunks.append(current) }
        return chunks
    }
}

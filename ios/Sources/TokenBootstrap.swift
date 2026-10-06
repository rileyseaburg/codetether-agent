import Foundation

/// Installation consumes a private container file, never a bundled secret.
enum TokenBootstrap {
    private struct Payload: Decodable { let token: String }

    static func consume() throws {
        let url = URL.documentsDirectory.appendingPathComponent("bootstrap.json")
        guard FileManager.default.fileExists(atPath: url.path) else { return }
        var protectedURL = url
        var values = URLResourceValues()
        values.isExcludedFromBackup = true
        try protectedURL.setResourceValues(values)
        try FileManager.default.setAttributes([.protectionKey: FileProtectionType.complete],
                                             ofItemAtPath: url.path)
        do {
            let data = try Data(contentsOf: url)
            let token = try JSONDecoder().decode(Payload.self, from: data).token
                .trimmingCharacters(in: .whitespacesAndNewlines)
            guard !token.isEmpty, token.rangeOfCharacter(from: .controlCharacters) == nil else {
                throw ClientError.missingToken
            }
            try TokenStore.save(token)
        } catch {
            try FileManager.default.removeItem(at: url)
            throw error
        }
        try FileManager.default.removeItem(at: url)
    }
}

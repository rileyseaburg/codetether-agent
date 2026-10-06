import Foundation
import Security

/// Non-synchronizing, device-only storage; no token in UserDefaults or bundles.
enum TokenStore {
    private static var query: [String: CFTypeRef] {
        [kSecClass as String: kSecClassGenericPassword,
         kSecAttrService as String: "run.codetether.ios.server" as CFString,
         kSecAttrAccount as String: "bearer" as CFString]
    }

    static func read() throws -> String? {
        var request = query
        request[kSecReturnData as String] = kCFBooleanTrue
        request[kSecMatchLimit as String] = kSecMatchLimitOne
        var result: CFTypeRef?
        let status = SecItemCopyMatching(request as CFDictionary, &result)
        if status == errSecItemNotFound { return nil }
        guard status == errSecSuccess, let data = result as? Data,
              let token = String(data: data, encoding: .utf8) else { throw ClientError.keychain(status) }
        return token
    }

    static func save(_ token: String) throws {
        let attributes: [String: CFTypeRef] = [
            kSecValueData as String: Data(token.utf8) as CFData,
            kSecAttrAccessible as String: kSecAttrAccessibleWhenUnlockedThisDeviceOnly]
        let status = SecItemUpdate(query as CFDictionary, attributes as CFDictionary)
        if status == errSecItemNotFound {
            let item = query.merging(attributes) { _, new in new }
            let added = SecItemAdd(item as CFDictionary, nil)
            guard added == errSecSuccess else { throw ClientError.keychain(added) }
        } else if status != errSecSuccess { throw ClientError.keychain(status) }
    }

    static func remove() throws {
        let status = SecItemDelete(query as CFDictionary)
        guard status == errSecSuccess || status == errSecItemNotFound else { throw ClientError.keychain(status) }
    }
}

import Foundation

struct ServerVersion: Decodable {
    let version: String
    let name: String
}

struct AgentProfile: Decodable, Identifiable {
    let name: String
    let description: String?
    let mode: String
    let hidden: Bool
    var id: String { name }
}

enum ClientError: LocalizedError {
    case missingToken, unauthorized, forbidden, http(Int), invalidResponse, keychain(Int32), agent(String)

    var errorDescription: String? {
        switch self {
        case .agent: return "The agent request failed. Retry, or start a new conversation."
        case .missingToken: return "Add your server bearer token in Settings."
        case .unauthorized: return "Token rejected (401). Update it in Settings."
        case .forbidden: return "This token does not have permission (403)."
        case .http(let status): return "The server returned HTTP \(status). Try again."
        case .invalidResponse: return "The server returned an unexpected response."
        case .keychain(let status):
            return "Unable to access the device Keychain (\(status)). Unlock and try again."
        }
    }
}

import Foundation

/// Sanitized errors: never display raw server bodies, headers, or bearer credentials.
enum ScreenFailure: Error, LocalizedError {
    case http(Int), invalidResponse, invalidEvent, disconnected, invalidInput

    var errorDescription: String? {
        switch self {
        case .http(401): return "Sign in again using Server settings, then reconnect."
        case .http(403): return "This login cannot access the screen session."
        case .http(404), .http(410): return "This screen session is unavailable or expired."
        case .http(412): return "Capture is paused until the iPhone stream reconnects."
        case .http: return "The screen service could not accept the request."
        case .invalidResponse, .invalidEvent: return "The screen service sent an unexpected response."
        case .disconnected: return "The screen stream disconnected."
        case .invalidInput: return "Choose a provider/model, up to 2,000 characters, and a 15–300 second interval."
        }
    }

    static func retryable(_ error: Error) -> Bool {
        if let failure = error as? ScreenFailure {
            switch failure {
            case .http(let code): return code == 408 || code == 429 || code >= 500
            case .disconnected: return true
            case .invalidResponse, .invalidEvent, .invalidInput: return false
            }
        }
        return error is URLError && (error as? URLError)?.code != .cancelled
    }
    static func message(_ error: Error) -> String {
        (error as? ScreenFailure)?.errorDescription ?? "Connection unavailable. Check Server settings and try again."
    }
}
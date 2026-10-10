import Foundation

/// Sanitized errors: never display raw server bodies, headers, or bearer credentials.
enum ScreenFailure: Error, LocalizedError {
    case http(Int), invalidResponse, invalidEvent, disconnected, invalidInput, invalidQuestion

    var errorDescription: String? {
        switch self {
        case .http(401): return "Sign in again using Server settings, then reconnect."
        case .http(403): return "This login cannot access the screen session."
        case .http(404), .http(410): return "This screen session is unavailable or expired."
        case .http(409): return "Pair Windows, or wait for the current capture, analysis, or queued reply to finish, then retry."
        case .http(412): return "Capture is paused until the iPhone stream reconnects."
        case .http(429): return "The capture or service limit was reached. Wait, or stop and create a new session."
        case .http: return "The screen service could not accept the request."
        case .invalidResponse, .invalidEvent: return "The screen service sent an unexpected response."
        case .disconnected: return "The screen stream disconnected."
        case .invalidInput: return "Choose a provider/model, up to 2,000 characters, and a 15–300 second interval."
        case .invalidQuestion: return "Enter a question or reply of 1–2,000 characters."
        }
    }

    static func retryable(_ error: Error) -> Bool {
        if let failure = error as? ScreenFailure {
            switch failure {
            case .http(let code): return code == 408 || code == 429 || code >= 500
            case .disconnected: return true
            case .invalidResponse, .invalidEvent, .invalidInput, .invalidQuestion: return false
            }
        }
        return error is URLError && (error as? URLError)?.code != .cancelled
    }
    static func message(_ error: Error) -> String {
        if let failure = error as? ScreenFailure { return failure.localizedDescription }
        if let failure = error as? ClientError { return failure.localizedDescription }
        return "Connection unavailable. Check Server settings and try again."
    }
}

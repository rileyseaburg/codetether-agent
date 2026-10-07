import SwiftUI

/// Status headline and live captions for `VoiceModeView` (split for
/// file-size limits).
extension VoiceModeView {
    var status: some View {
        VStack(spacing: 4) {
            Text(headline).font(.headline)
            VoicePausePicker(loop: loop)
            if !loop.vadAvailable {
                Text("Voice meter unavailable. Auto-send still waits for your last word.")
                    .font(.caption).foregroundStyle(.secondary)
            }
        }
        .accessibilityIdentifier("voice-status")
    }

    var headline: String {
        switch loop.phase {
        case .idle: "Tap to start"
        case .listening: "Listening…"
        case .userSpeaking: "Go ahead…"
        case .thinking: "Thinking…"
        case .speaking: "CodeTether is speaking — tap to interrupt"
        case .failed: "Something went wrong"
        }
    }

    var captions: some View {
        VStack(spacing: 8) {
            if !loop.transcript.isEmpty {
                Text(loop.transcript).font(.body).multilineTextAlignment(.center)
                    .accessibilityIdentifier("voice-user-caption")
            }
            if let reply = loop.reply {
                Text(reply).font(.body).multilineTextAlignment(.center)
                    .foregroundStyle(.secondary)
                    .accessibilityIdentifier("voice-reply-caption")
            }
        }
    }
}
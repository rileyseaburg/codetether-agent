import SwiftUI

/// Phase-driven visuals for the talking orb: colors, icons, sizing,
/// animations, and the mic-level scale factor.
extension TalkingOrb {
    var color: Color {
        switch phase {
        case .idle: .gray
        case .listening: .cyan
        case .userSpeaking: .cyan
        case .thinking: .indigo
        case .speaking: .green
        case .failed: .red
        }
    }

    var iconName: String {
        switch phase {
        case .idle: "mic.fill"
        case .listening: "waveform"
        case .userSpeaking: "waveform"
        case .thinking: "brain.head.profile"
        case .speaking: "waveform"
        case .failed: "exclamationmark.triangle"
        }
    }

    var size: CGFloat { 150 }
    var ease: Animation { .easeInOut(duration: 2.2) }
    var linear: Animation { .linear(duration: 1.4) }

    /// -60..0 dBFS mic level → 0..1, nudging orb scale while speaking.
    var scale: CGFloat {
        let level = max(0, min(1, (levelDb + 60) / 60))
        let base: Float = phase == .userSpeaking || phase == .speaking ? 1.0 : 0.9
        let bump: Float = phase == .userSpeaking ? Float(level) * 0.18 : 0
        return CGFloat(base + bump)
    }

    var accessibilityText: String {
        switch phase {
        case .idle: "Tap to start voice mode"
        case .listening: "Listening. Tap to send now."
        case .userSpeaking: "Listening to you. Tap to send now."
        case .thinking: "Thinking. Tap to stop."
        case .speaking: "Speaking. Tap to interrupt."
        case .failed: "Voice mode failed"
        }
    }
}

import SwiftUI

/// Hands-free pause length, persisted between visits to the Voice tab.
struct VoicePausePicker: View {
    @ObservedObject var loop: VoiceLoop
    var body: some View {
        VStack(spacing: 6) {
            Text("Send after a pause").font(.caption).foregroundStyle(.secondary)
            Picker("Pause after last word", selection: $loop.pauseSeconds) {
                Text("3 seconds").tag(3)
                Text("5 seconds").tag(5)
            }
            .pickerStyle(.segmented).frame(maxWidth: 260)
            .accessibilityIdentifier("voice-pause-duration")
            Text("Speak naturally. Listening resumes after each reply.")
                .font(.caption).foregroundStyle(.secondary)
            Button("End voice session", role: .destructive) { loop.stop() }
                .font(.caption).accessibilityIdentifier("voice-end-session")
        }
    }
}
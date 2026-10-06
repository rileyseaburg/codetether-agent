import SwiftUI

struct VoiceControls: View {
    @Environment(\.scenePhase) private var phase
    @ObservedObject var chat: ChatModel
    @ObservedObject var input: VoiceInput
    @ObservedObject var output: VoiceOutput
    var body: some View {
        VStack(spacing: 6) {
            HStack {
                Button {
                    if input.listening { input.stop() }
                    else { output.stop(); Task { await input.start { chat.draft = $0 } } }
                } label: { Label(input.listening ? "Stop mic" : "Dictate", systemImage: input.listening ? "mic.slash.fill" : "mic.fill") }
                .disabled(chat.busy).accessibilityIdentifier("voice-input")
                Spacer()
                Toggle("Read aloud", isOn: $output.enabled).fixedSize().accessibilityIdentifier("read-aloud-toggle")
            }
            HStack {
                Button("Test speaker") { input.stop(); output.speak("Hello Riley. This is your Kokoro voice speaking from the CodeTether server.") }
                    .accessibilityIdentifier("test-speaker")
                Spacer()
                if output.speaking { Button("Stop audio") { output.stop() } }
            }
            if !output.status.isEmpty {
                Text(output.status).font(.caption).accessibilityIdentifier("voice-status")
            }
            if input.listening { Text("Listening — tap Stop mic, then Send").font(.caption) }
            if let error = input.error { Text(error).font(.caption).foregroundStyle(.red) }
        }.padding(.horizontal).padding(.vertical, 6)
            .onChange(of: phase) { _, phase in if phase != .active { input.stop() } }
    }
}

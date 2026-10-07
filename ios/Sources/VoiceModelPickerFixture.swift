#if DEBUG && targetEnvironment(simulator)
import SwiftUI

/// Real Voice selector UI with fixture model names; never starts a microphone or server turn.
struct VoiceModelPickerFixture: View {
    @StateObject private var chat = ChatModel()
    @StateObject private var loop = VoiceLoop()
    @StateObject private var mic = VoiceMicEngine()
    @State private var creating = false
    var body: some View {
        VoiceSessionScreen(chat: chat, loop: loop, mic: mic, creating: $creating)
            .onAppear {
                chat.models = ["provider/first", "provider/second"]
                chat.selectedModel = "provider/first"
            }
    }
}
#endif




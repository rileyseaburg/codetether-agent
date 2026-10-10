import SwiftUI
import UIKit

@main
struct CodeTetherApp: App {
    @StateObject private var connection = ConnectionModel()
    @StateObject private var chat = ChatModel()
    @StateObject private var screen = ScreenModel()
    @Environment(\.scenePhase) private var scenePhase

    init() {
        if CommandLine.arguments.contains("--uitesting") { UIView.setAnimationsEnabled(false) }
    }
    var body: some Scene {
        WindowGroup {
#if DEBUG && targetEnvironment(simulator)
            if CommandLine.arguments.contains("--transcript-scroll-fixture") {
                TranscriptScrollFixture()
            } else if CommandLine.arguments.contains("--voice-model-picker-fixture") {
                VoiceModelPickerFixture()
            } else if CommandLine.arguments.contains("--markdown-copy-fixture") {
                MarkdownCopyFixture()
            } else { appContent }
#else
            appContent
#endif
        }
    }

    private var appContent: some View {
        TabView {
            ChatView(connection: connection, chat: chat)
                .tabItem { Label("Chat", systemImage: "bubble.left.and.bubble.right") }
            VoiceModeTab(chat: chat)
                .tabItem { Label("Voice", systemImage: "waveform.circle") }
            ScreenView(model: screen)
                .tabItem { Label("Screen", systemImage: "desktopcomputer") }
            DashboardView(model: connection)
                .tabItem { Label("Server", systemImage: "server.rack") }
        }
        .privacySensitive()
        .overlay {
            if scenePhase != .active {
                Color(.systemBackground).ignoresSafeArea()
                    .overlay(Image(systemName: "lock.shield").font(.largeTitle))
            }
        }
    }
}
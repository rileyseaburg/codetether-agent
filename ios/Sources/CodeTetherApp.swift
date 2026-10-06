import SwiftUI
import UIKit

@main
struct CodeTetherApp: App {
    @StateObject private var connection = ConnectionModel()
    @Environment(\.scenePhase) private var scenePhase

    init() {
        if CommandLine.arguments.contains("--uitesting") { UIView.setAnimationsEnabled(false) }
    }
    var body: some Scene {
        WindowGroup {
#if DEBUG && targetEnvironment(simulator)
            if CommandLine.arguments.contains("--transcript-scroll-fixture") {
                TranscriptScrollFixture()
            } else { appContent }
#else
            appContent
#endif
        }
    }

    private var appContent: some View {
        TabView {
            ChatView(connection: connection)
                .tabItem { Label("Chat", systemImage: "bubble.left.and.bubble.right") }
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
import SwiftUI

/// Read-only Windows observation, separate from chat and its tool-capable session.
struct ScreenView: View {
    @ObservedObject var model: ScreenModel
    @ObservedObject var chat: ChatModel
    @Environment(\.scenePhase) private var scenePhase
    @State private var visible = false

    var body: some View {
        NavigationStack {
            Form {
                ScreenSetupView(model: model, chat: chat)
                if let session = model.session {
                    ScreenPairingView(session: session, status: model.response.status)
                    ScreenResponseView(model: model)
                }
                if let error = model.error {
                    Section("Connection") { Text(error).foregroundStyle(.red) }
                }
                Section {
                    Text("Keep this Screen tab open to receive live analysis. Backgrounding or leaving this tab pauses uploads. Screenshots go to your selected AI provider; close private windows first.")
                        .font(.footnote).foregroundStyle(.secondary)
                }
            }
            .navigationTitle("Windows Screen")
        }
        .onAppear { visible = true; model.setActive(scenePhase == .active) }
        .onDisappear { visible = false; model.setActive(false) }
        .onChange(of: scenePhase) { _, phase in model.setActive(visible && phase == .active) }
        .task { if chat.models.isEmpty { await chat.loadModels() } }
    }
}
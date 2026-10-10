import SwiftUI

/// Screen assistance and directly requested typing stay separate from agent chat.
struct ScreenView: View {
    @ObservedObject var model: ScreenModel
    @StateObject private var selection = ScreenModelSelection()
    @Environment(\.scenePhase) private var scenePhase
    @State private var visible = false
    @State private var confirmingStop = false

    var body: some View {
        NavigationStack {
            Form {
                ScreenSetupView(model: model, selection: selection)
                if let session = model.session {
                    ScreenPairingView(session: session, status: model.response.status)
                    ScreenStatusView(model: model)
                    ScreenCaptureButton(model: model, questions: model.questions)
                    ScreenAnalysisView(response: model.response)
                    ScreenQuestionView(model: model, questions: model.questions)
                }
                if let error = model.error {
                    Section { Text(error).foregroundStyle(.red).accessibilityIdentifier("screen-error") }
                }
                Section { Text(model.notice).font(.footnote).foregroundStyle(.secondary) }
                if model.session != nil {
                    Section {
                        Button(model.stopping ? "Stopping…" : "Stop session", role: .destructive) {
                            confirmingStop = true
                        }.disabled(model.stopping).accessibilityIdentifier("screen-stop")
                    }
                }
            }
            .navigationTitle("Screen")
            .confirmationDialog("Stop sharing and revoke Windows access?", isPresented: $confirmingStop) {
                Button("Stop session", role: .destructive) { Task { await model.stop() } }
            }
            .task { await selection.load() }
            .onAppear { visible = true; model.setActive(scenePhase == .active) }
            .onDisappear { visible = false; model.setActive(false) }
            .onChange(of: scenePhase) { _, phase in model.setActive(visible && phase == .active) }
        }
    }
}

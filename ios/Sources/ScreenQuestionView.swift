import SwiftUI

/// Asks about a fresh screenshot. Text goes only to the owner relay endpoint.
struct ScreenQuestionView: View {
    @ObservedObject var model: ScreenModel
    @ObservedObject var questions: ScreenQuestionState
    @StateObject private var dictation = ScreenDictation()
    @Environment(\.scenePhase) private var phase

    var body: some View {
        Section("Ask about the current screen") {
            TextField("What should AI check on screen now?", text: $questions.draft, axis: .vertical)
                .lineLimit(2...5).disabled(questions.submitting)
                .accessibilityIdentifier("screen-question")
            HStack {
                Button {
                    Task { await dictation.toggle { questions.draft = $0 } }
                } label: {
                    Label(dictation.listening ? "Stop" : "Dictate",
                          systemImage: dictation.listening ? "mic.slash.fill" : "mic.fill")
                }.buttonStyle(.bordered).disabled(questions.submitting)
                Spacer()
                Button("Type on Windows") {
                    dictation.stop(); model.askModelToType()
                }
                .buttonStyle(.bordered)
                .disabled(!model.canAskQuestion || !ScreenInput.text(questions.draft))
                .accessibilityIdentifier("screen-type")
                Button(questions.submitting ? "Sending…" : "Ask AI") {
                    dictation.stop(); model.askQuestion()
                }
                .buttonStyle(.borderedProminent)
                .disabled(!model.canAskQuestion || !ScreenInput.text(questions.draft))
                .accessibilityIdentifier("screen-ask")
            }
            Text("Ask AI answers about a fresh screenshot. Type on Windows has the AI write text from your instruction and the screenshot, then types it once at the focused Windows field. No clicking, Enter or submission.")
                .font(.caption).foregroundStyle(.secondary)
            if let notice = questions.notice { Text(notice).font(.footnote) }
            if let error = questions.error ?? dictation.error {
                Text(error).font(.footnote).foregroundStyle(.red)
            }
        }
        .onChange(of: phase) { _, value in if value != .active { dictation.stop() } }
        .onDisappear { dictation.stop() }
    }
}

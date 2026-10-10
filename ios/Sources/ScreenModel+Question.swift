import Foundation

extension ScreenModel {
    var canAskQuestion: Bool { questionBlockReason == nil }

    /// Fresh capture answered with the typed or dictated question.
    func askQuestion() { requestFrame(questions.draft, clearsDraft: true) }

    /// Fresh capture answered with the session's standing instructions.
    func captureNow() { requestFrame(prompt, clearsDraft: false) }

    /// "Type on Windows": the model composes the keyboard text from the owner's
    /// instruction and a fresh screenshot; the raw draft is never typed verbatim.
    func askModelToType() {
        let instruction = questions.draft.trimmingCharacters(in: .whitespacesAndNewlines)
        let text = "Type into the focused Windows text field. Compose the text from this instruction: \(instruction)"
        guard !instruction.isEmpty, ScreenInput.text(text) else {
            questions.error = "Use a nonblank instruction that fits the 2,000-character limit."; return
        }
        questions.error = nil
        requestFrame(text, clearsDraft: true, draft: questions.draft)
    }

    /// Queues a fresh frame; explicit typing requests authorize relay keyboard delivery, not chat/tools.
    private func requestFrame(_ text: String, clearsDraft: Bool, draft: String? = nil) {
        guard canAskQuestion, let owned = session,
              let requester = client as? ScreenQuestionNetworking else { return }
        let body = ScreenQuestion(question: text)
        guard ScreenInput.text(body.question) else {
            questions.error = ScreenFailure.invalidQuestion.localizedDescription; return
        }
        let sequence = response.sequence
        questions.cancel()
        questions.submitting = true
        let ticket = questions.generation
        questions.task = Task { [weak self] in
            guard let self else { return }
            defer {
                if questions.generation == ticket { questions.submitting = false; questions.task = nil }
            }
            do {
                _ = try await requester.ask(body, session: owned.id, token: credential())
                guard !Task.isCancelled, questions.generation == ticket, session?.id == owned.id else { return }
                if clearsDraft, questions.draft == (draft ?? body.question) { questions.draft = "" }
                questions.waitingForFrame = response.sequence == sequence || response.status == "requested"
                questions.notice = questions.waitingForFrame ? "Waiting for a fresh screenshot from Windows…" : nil
            } catch {
                guard !Task.isCancelled, questions.generation == ticket, session?.id == owned.id else { return }
                questions.error = ScreenFailure.message(error)
                questions.notice = "No automatic retry was sent. Check live analysis before trying again."
            }
        }
    }
}

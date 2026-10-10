import Foundation

extension ScreenModel {
    /// Start exactly once; foreground changes do not cancel an in-flight creation.
    func start(model: String) async {
        guard !locked else { return }
        let body = ScreenCreateBody(model: model, prompt: prompt, interval_seconds: interval)
        guard body.valid else { error = ScreenFailure.invalidInput.localizedDescription; return }
        creating = true
        error = nil
        invalidateStream()
        let ticket = generation
        do {
            let receipt = try await client.create(body, token: credential())
            guard generation == ticket else { creating = false; return }
            session = receipt
            activeModel = model
            response = ScreenResponse()
            questions.cancel(clearDraft: true)
            replies.cancel(clearDraft: true)
            retryBlocked = !receipt.valid
            if !receipt.valid { throw ScreenFailure.invalidResponse }
            scheduleExpiry(for: receipt)
            notice = "Pair Windows using the one-use code below."
        } catch { self.error = ScreenFailure.message(error) }
        creating = false
        connect()
    }

    /// A failed DELETE keeps ownership and Stop remains available for retry.
    func stop() async {
        guard let owned = session, !stopping, !creating else { return }
        stopping = true
        invalidateStream()
        retryBlocked = true
        error = nil
        do {
            try await client.stop(owned.id, token: credential())
            stopping = false
            guard session?.id == owned.id else { return }
            finishSession("Stopped. Windows pairing and capture are revoked.")
        } catch {
            stopping = false
            self.error = ScreenFailure.message(error)
            notice = "Stop was not confirmed. Pause on Windows, then retry Stop."
        }
    }
}

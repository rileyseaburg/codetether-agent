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
            guard generation == ticket else { return }
            session = receipt
            activeModel = model
            response = ScreenResponse()
            retryBlocked = !receipt.valid
            if !receipt.valid { throw ScreenFailure.invalidResponse }
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
        let ticket = generation
        retryBlocked = true
        error = nil
        do {
            try await client.stop(owned.id, token: credential())
            guard generation == ticket else { return }
            session = nil
            notice = "Stopped. Windows pairing and capture are revoked."
            retryBlocked = false
        } catch {
            self.error = ScreenFailure.message(error)
            notice = "Capture paused. Stop was not confirmed; retry Stop."
        }
        stopping = false
    }
}
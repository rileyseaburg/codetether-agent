import Foundation
import Combine

/// Screen choices are independent of the tool-capable chat's selected model.
@MainActor
final class ScreenModelSelection: ObservableObject {
    @Published var selected = UserDefaults.standard.string(forKey: "screen.model") ?? "" {
        didSet { UserDefaults.standard.set(selected, forKey: "screen.model") }
    }
    @Published private(set) var models: [String] = []
    @Published private(set) var loading = false
    @Published private(set) var error: String?
    private let client = ServerClient()

    func load() async {
        guard !loading else { return }
        loading = true
        error = nil
        defer { loading = false }
        do {
            guard let token = try TokenStore.read() else { throw ClientError.missingToken }
            let catalog: ModelCatalog = try await client.get("v1/models", token: token)
            try Task.checkCancellation()
            models = Array(Set(catalog.data.map(\.id).filter(ScreenInput.model))).sorted()
        } catch is CancellationError {
            return
        } catch {
            self.error = ScreenFailure.message(error)
        }
    }
}

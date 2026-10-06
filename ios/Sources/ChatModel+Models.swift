import Foundation

extension ChatModel {
    func loadModels() async {
        guard !loading else { return }
        loading = true
        defer { loading = false }
        do {
            guard let token = try TokenStore.read() else { throw ClientError.missingToken }
            let catalog: ModelCatalog = try await client.get("v1/models", token: token)
            models = Array(Set(catalog.data.map(\.id))).sorted()
            guard !models.isEmpty else { error = "No models are configured on the server."; return }
            if !models.contains(selectedModel) {
                let config: ChatConfiguration? = try? await client.get("api/config", token: token)
                let saved = UserDefaults.standard.string(forKey: "chat.model")
                selectedModel = ChatModelSelection.choose(models: models, saved: saved,
                                                          serverDefault: config?.default_model)
            }
            error = nil
        } catch {
            self.error = (error as? ClientError)?.localizedDescription ?? "Unable to load models. Check your connection and retry."
        }
    }
}

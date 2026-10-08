import Foundation

/// Injectable, owner-authenticated transport; tests never invoke a model or the live relay.
@MainActor
protocol ScreenNetworking {
    func create(_ body: ScreenCreateBody, token: String) async throws -> ScreenSession
    func stop(_ id: UUID, token: String) async throws
    func events(_ id: UUID, token: String,
                receive: @escaping @MainActor (ScreenEvent) -> Void) async throws
}

/// Cancellation-aware backoff injection keeps lifecycle tests deterministic.
typealias ScreenDelay = @MainActor (UInt64) async throws -> Void
typealias ScreenToken = @MainActor () throws -> String?
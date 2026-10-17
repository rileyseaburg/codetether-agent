//! into service task-store operation.
use super::*;
impl GrpcTaskStore {
    /// Build the tonic service layer from this store.
    ///
    /// Wraps the shared task store in an [`A2aServiceImpl`] and returns the
    /// generated tonic [`A2aServiceServer`] ready to be mounted into a gRPC
    /// server.
    ///
    /// # Returns
    ///
    /// A tonic service server that handles A2A gRPC requests using this store as
    /// its backing state.
    ///
    /// # Side effects
    ///
    /// Consumes one [`Arc`] handle to the store and moves it into the service
    /// implementation. This method does not start listening on a socket; the
    /// caller is responsible for adding the returned service to a tonic server.
    pub fn into_service(self: Arc<Self>) -> A2aServiceServer<A2aServiceImpl> {
        A2aServiceServer::new(A2aServiceImpl { store: self })
    }
}

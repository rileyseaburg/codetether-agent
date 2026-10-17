use super::*;
impl JsonRpcError {
    /// Construct the corresponding JSON-RPC dispatch error.
    ///
    /// # Arguments
    /// * `method` - Method name or error message.
    ///
    /// # Returns
    /// An error with the standard code and no data.
    ///
    /// # Examples
    /// ```
    /// let error = codetether_a2a_core::JsonRpcError::method_not_found("unavailable");
    /// assert!(error.data.is_none());
    /// ```
    pub fn method_not_found(method: &str) -> Self {
        Self {
            code: METHOD_NOT_FOUND,
            message: format!("Method not found: {}", method),
            data: None,
        }
    }

    /// Construct the corresponding JSON-RPC dispatch error.
    ///
    /// # Arguments
    /// * `msg` - Method name or error message.
    ///
    /// # Returns
    /// An error with the standard code and no data.
    ///
    /// # Examples
    /// ```
    /// let error = codetether_a2a_core::JsonRpcError::internal_error("unavailable");
    /// assert!(error.data.is_none());
    /// ```
    pub fn internal_error(msg: impl Into<String>) -> Self {
        Self {
            code: INTERNAL_ERROR,
            message: msg.into(),
            data: None,
        }
    }

    /// Construct the corresponding JSON-RPC dispatch error.
    ///
    /// # Arguments
    /// * `msg` - Method name or error message.
    ///
    /// # Returns
    /// An error with the standard code and no data.
    ///
    /// # Examples
    /// ```
    /// let error = codetether_a2a_core::JsonRpcError::unsupported_operation("unavailable");
    /// assert!(error.data.is_none());
    /// ```
    pub fn unsupported_operation(msg: impl Into<String>) -> Self {
        Self {
            code: UNSUPPORTED_OPERATION,
            message: msg.into(),
            data: None,
        }
    }
}

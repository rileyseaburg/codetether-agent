use super::*;
impl JsonRpcError {
    /// Construct the corresponding JSON-RPC client error.
    ///
    /// # Arguments
    /// * `msg` - Error message to retain.
    ///
    /// # Returns
    /// An error with the standard code and no data.
    ///
    /// # Examples
    /// ```
    /// let error = codetether_a2a_core::JsonRpcError::parse_error("invalid request");
    /// assert!(error.data.is_none());
    /// ```
    pub fn parse_error(msg: impl Into<String>) -> Self {
        Self {
            code: PARSE_ERROR,
            message: msg.into(),
            data: None,
        }
    }

    /// Construct the corresponding JSON-RPC client error.
    ///
    /// # Arguments
    /// * `msg` - Error message to retain.
    ///
    /// # Returns
    /// An error with the standard code and no data.
    ///
    /// # Examples
    /// ```
    /// let error = codetether_a2a_core::JsonRpcError::invalid_request("invalid request");
    /// assert!(error.data.is_none());
    /// ```
    pub fn invalid_request(msg: impl Into<String>) -> Self {
        Self {
            code: INVALID_REQUEST,
            message: msg.into(),
            data: None,
        }
    }

    /// Construct the corresponding JSON-RPC client error.
    ///
    /// # Arguments
    /// * `msg` - Error message to retain.
    ///
    /// # Returns
    /// An error with the standard code and no data.
    ///
    /// # Examples
    /// ```
    /// let error = codetether_a2a_core::JsonRpcError::invalid_params("invalid request");
    /// assert!(error.data.is_none());
    /// ```
    pub fn invalid_params(msg: impl Into<String>) -> Self {
        Self {
            code: INVALID_PARAMS,
            message: msg.into(),
            data: None,
        }
    }
}

use crate::*;

/// Represents a parsed HTTP response.
#[derive(Debug, Clone, PartialEq, Eq, Data, DisplayDebug)]
pub struct Response {
    /// The HTTP version used in the response.
    pub(super) version: ResponseVersion,
    /// The HTTP status code.
    pub(super) status_code: ResponseStatusCode,
    /// The reason phrase associated with the status code.
    #[set(skip)]
    pub(super) reason_phrase: ResponseReasonPhrase,
    /// The response headers as key-value pairs.
    #[set(skip)]
    pub(super) headers: ResponseHeaders,
    /// The binary body content of the response.
    #[set(skip)]
    pub(super) body: ResponseBody,
}

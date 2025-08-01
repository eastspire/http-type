use crate::*;

/// Represents the components of a parsed HTTP URL.
///
/// This struct holds various parts of a URL, including the protocol, host, port,
/// path, query, and fragment, allowing for structured access to URL information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpUrlComponents {
    /// The URL scheme, such as "http" or "https".
    pub protocol: Protocol,
    /// The host part of the URL.
    pub host: OptionString,
    /// The port number in the URL, if specified.
    pub port: OptionU16,
    /// The path in the URL.
    pub path: OptionString,
    /// The query string in the URL.
    pub query: OptionString,
    /// The fragment identifier.
    pub fragment: OptionString,
}

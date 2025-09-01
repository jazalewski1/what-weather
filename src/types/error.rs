#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FetchError {
    ConnectionFailure,
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::ConnectionFailure => "Failed to connect to external server",
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for FetchError {}

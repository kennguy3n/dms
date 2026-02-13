#[derive(Debug)]
pub enum SdkError {
    Transport(String),
    Serialization(String),
    Auth(String),
    Api { status: u16, message: String },
    InvalidInput(String),
}

pub type SdkResult<T> = Result<T, SdkError>;

impl std::fmt::Display for SdkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transport(msg) => write!(f, "transport error: {msg}"),
            Self::Serialization(msg) => write!(f, "serialization error: {msg}"),
            Self::Auth(msg) => write!(f, "auth error: {msg}"),
            Self::Api { status, message } => write!(f, "api error {status}: {message}"),
            Self::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
        }
    }
}

impl std::error::Error for SdkError {}

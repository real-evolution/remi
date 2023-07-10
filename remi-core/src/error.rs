use thiserror::Error;

/// A type alias for results returned by remi.
pub type Result<T> = std::result::Result<T, Error>;

/// A type to rerpesent errors returned by remi.
#[derive(Error, Debug)]
pub enum Error {
    /// An unknown error occurred.
    #[error("unknown error: {0}")]
    Unknown(#[from] Box<dyn std::error::Error + Send + Sync>),

    /// An error returned by the underlying transport.
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    /// An error returned when protocol-violating data is detected.
    #[error("invalid format: {0}")]
    InvalidFormat(&'static str),

    /// An error returned when invalid frames are detected.
    #[error("invalid frame: {0}")]
    InvalidFrame(&'static str),
}

use thiserror::Error;

/// A type alias for results returned by remi.
pub type RemiResult<T> = Result<T, RemiError>;

/// A type to rerpesent errors returned by remi.
#[derive(Error, Debug)]
pub enum RemiError {
    /// An error returned by the underlying transport.
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
}

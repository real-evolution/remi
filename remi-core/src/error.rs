use thiserror::Error;

/// A type alias for results returned by remi.
pub type Result<T> = std::result::Result<T, Error>;

/// A type to rerpesent errors returned by remi.
#[derive(Error, Debug)]
pub enum Error {

}

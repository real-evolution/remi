#![feature(trait_alias)]
#![feature(impl_trait_in_assoc_type)]

mod conn;
mod error;
mod instance;
mod server;

pub use error::{Error, Result};
pub use server::RSocketServer;

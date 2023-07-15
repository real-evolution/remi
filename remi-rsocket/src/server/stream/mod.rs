pub mod builder;
mod service;

use std::ops::Deref;

use super::RSocketServer;

/// A variant of [`RSocketServer`] for stream transport.
#[derive(Debug, Clone)]
pub struct RSocketStreamServer {
    inner: RSocketServer,
}

impl Deref for RSocketStreamServer {
    type Target = RSocketServer;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

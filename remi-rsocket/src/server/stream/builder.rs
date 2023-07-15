#[derive(Debug)]
pub struct RSocketStreamServerBuilder {
    inner: super::RSocketServer,
}

impl RSocketStreamServerBuilder {
    /// Creates a new [`RSocketStreamServerBuilder`] with the given
    /// [`RSocketServer`].
    ///
    /// # Parameters
    /// - `inner` - The [`RSocketServer`] to use.
    #[inline]
    pub(crate) const fn new(inner: super::RSocketServer) -> Self {
        Self { inner }
    }

    /// Builds the [`RSocketStreamServer`].
    #[inline]
    pub const fn build(self) -> super::RSocketStreamServer {
        super::RSocketStreamServer { inner: self.inner }
    }
}

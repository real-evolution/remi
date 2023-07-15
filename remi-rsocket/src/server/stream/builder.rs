#[derive(Debug)]
pub struct RSocketStreamServerBuilder {
    inner: super::RSocketServer,
}

impl RSocketStreamServerBuilder {
    #[inline]
    pub(crate) const fn new(inner: super::RSocketServer) -> Self {
        Self { inner }
    }

    #[inline]
    pub const fn build(self) -> super::RSocketStreamServer {
        super::RSocketStreamServer { inner: self.inner }
    }
}

use std::time::Duration;

use super::stream::builder::RSocketStreamServerBuilder;

const DEFAULT_MAX_LIFETIME: Duration = Duration::from_secs(90);

/// A top-level builder for [`RSocketServer`] and its variants.
#[derive(Debug)]
pub struct RSocketServerBuilder {
    max_lifetime: Option<Duration>,
}

impl RSocketServerBuilder {
    #[inline]
    pub(super) fn new() -> Self {
        Self { max_lifetime: None }
    }

    /// Set the maximum lifetime of connections accepted by the server.
    ///
    /// # Parameters
    /// * `max_lifetime` - The maximum lifetime of connections.
    #[inline]
    pub fn max_lifetime(mut self, max_lifetime: Duration) -> Self {
        self.max_lifetime = Some(max_lifetime);
        self
    }

    /// Build a [`RSocketStreamServerBuilder`] from this builder to build a
    /// stream-based server.
    #[inline]
    pub fn stream(self) -> RSocketStreamServerBuilder {
        RSocketStreamServerBuilder::new(self.build())
    }

    #[inline]
    fn build(self) -> super::RSocketServer {
        super::RSocketServer {
            max_lifetime: self.max_lifetime.unwrap_or(DEFAULT_MAX_LIFETIME),
        }
    }
}

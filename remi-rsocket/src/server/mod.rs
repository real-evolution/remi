use std::time::Duration;

mod stream;
mod builder;
mod util;

pub use self::builder::RSocketServerBuilder;

#[derive(Debug, Clone)]
pub struct RSocketServer {
    max_lifetime: Duration,
}

impl RSocketServer {
    #[inline]
    pub const fn max_lifetime(&self) -> Duration {
        self.max_lifetime
    }

    #[inline]
    pub fn builder() -> builder::RSocketServerBuilder {
        builder::RSocketServerBuilder::new()
    }
}

/// A type to represent a message.
///
/// # Type Parameters
/// - `M`: The type of the message metadata.
/// - `B`: The type of the message body.
pub struct Message<M, B> {
    meta: M,
    data: B,
}

impl<M, B> Message<M, B> {
    /// Creates a new message.
    ///
    /// # Parameters
    /// - `meta`: The metadata.
    /// - `data`: The body.
    #[inline]
    pub const fn new(meta: M, data: B) -> Self {
        Self { meta, data }
    }

    /// Returns a reference to the metadata.
    #[inline]
    pub const fn metadata(&self) -> &M {
        &self.meta
    }

    /// Returns a reference to the body.
    #[inline]
    pub const fn data(&self) -> &B {
        &self.data
    }

    /// Deconstructs the message into its metadata and body pair.
    #[inline]
    pub fn split(self) -> (M, B) {
        (self.meta, self.data)
    }
}

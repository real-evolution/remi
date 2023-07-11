/// A type to represent a request.
///
/// # Type Parameters
/// - `M`: The type of the request metadata.
/// - `B`: The type of the request body.
#[derive(Debug)]
pub struct Request<M, B> {
    meta: M,
    data: B,
}

impl<M, B> Request<M, B> {
    /// Creates a new request.
    ///
    /// # Parameters
    /// - `meta`: The metadata.
    /// - `data`: The body.
    #[inline]
    pub const fn new(meta: M, data: B) -> Self {
        Self { meta, data }
    }

    /// Deconstructs the request into its metadata and body pair.
    #[inline]
    pub fn split(self) -> (M, B) {
        (self.meta, self.data)
    }
}

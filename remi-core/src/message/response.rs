/// A type to represent a response.
///
/// # Type Parameters
/// - `M`: The type of the response metadata.
/// - `B`: The type of the response body.
#[derive(Debug)]
pub struct Response<M, B> {
    meta: M,
    data: B,
}

impl<M, B> Response<M, B> {
    /// Creates a new response.
    ///
    /// # Parameters
    /// - `meta`: The metadata.
    /// - `data`: The body.
    #[inline]
    pub const fn new(meta: M, data: B) -> Self {
        Self { meta, data }
    }

    /// Deconstructs the response into its metadata and body pair.
    #[inline]
    pub fn split(self) -> (M, B) {
        (self.meta, self.data)
    }
}

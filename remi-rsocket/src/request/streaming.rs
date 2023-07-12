use remi_util::channel;

/// A type to represent a streaming request with a streaming response.
#[derive(Debug)]
pub struct StreamingRequest<Request, Response> {
    request: channel::multi::MultiConsumer<Request>,
    response: channel::multi::MultiProducer<Response>,
}

impl<Request, Response> StreamingRequest<Request, Response> {
    /// Create a new bidirectional streaming request with an optional bound on
    /// the buffer.
    ///
    /// # Parameters
    /// * `bound` - (optional) The maximum number of responses to buffer.
    #[inline]
    pub fn new(
        bound: Option<usize>,
    ) -> (Self, channel::duplix::DuplixSide<Request, Response>) {
        let (e1, e2) = channel::duplix::maybe_bounded(bound);

        (Self::from_duplix_side(e2), e1)
    }

    /// Split the request and response sides of the streaming request.
    #[inline]
    pub fn split(
        self,
    ) -> (
        channel::multi::MultiConsumer<Request>,
        channel::multi::MultiProducer<Response>,
    ) {
        (self.request, self.response)
    }

    #[inline]
    fn from_duplix_side(
        side: channel::duplix::DuplixSide<Response, Request>,
    ) -> Self {
        let (response, request) = side.split();

        Self { request, response }
    }
}

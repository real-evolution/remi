use remi_util::channel;

/// A type to represent a request with a streaming response.
#[derive(Debug)]
pub struct ServerStreamingRequest<Request, Response> {
    request: Request,
    response: channel::multi::MultiProducer<Response>,
}

impl<Request, Response> ServerStreamingRequest<Request, Response> {
    /// Create a new server streaming request with an optional bound on the
    /// response buffer.
    ///
    /// # Parameters
    /// * `request` - The request to send.
    /// * `bound` - (optional) The maximum number of responses to buffer.
    #[inline]
    pub fn new(
        request: Request,
        bound: Option<usize>,
    ) -> (Self, channel::multi::MultiConsumer<Response>) {
        let (tx, rx) = channel::multi::maybe_bounded(bound);
        let req = Self {
            request,
            response: tx,
        };

        (req, rx)
    }

    /// Split the request and response sides of the request.
    #[inline]
    pub fn split(self) -> (Request, channel::multi::MultiProducer<Response>) {
        (self.request, self.response)
    }
}

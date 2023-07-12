use remi_util::channel;

/// A type to represent a request with a streaming response.
#[derive(Debug)]
pub struct ClientStreamingRequest<Request, Response> {
    request: channel::multi::MultiConsumer<Request>,
    response: channel::unary::UnaryProducer<Response>,
}

impl<Request, Response> ClientStreamingRequest<Request, Response> {
    /// Create a new server streaming request with a *bounded* channel to
    /// receive the server's responses.
    ///
    /// # Parameters
    /// * `request` - The request to send.
    /// * `bound` - The maximum number of responses to buffer.
    #[inline]
    pub fn new(
        bound: Option<usize>,
    ) -> (Self, ClientStreamingHandle<Request, Response>) {
        let (req_tx, req_rx) = channel::multi::maybe_bounded(bound);
        let (res_tx, res_rx) = channel::unary::unary();

        let req = Self {
            request: req_rx,
            response: res_tx,
        };

        let handle = ClientStreamingHandle {
            request: req_tx,
            response: res_rx,
        };

        (req, handle)
    }

    /// Split the request and response sides of the request.
    #[inline]
    pub fn split(
        self,
    ) -> (
        channel::multi::MultiConsumer<Request>,
        channel::unary::UnaryProducer<Response>,
    ) {
        (self.request, self.response)
    }
}

/// A type to represent a handle to a client streaming request producer side.
#[derive(Debug)]
pub struct ClientStreamingHandle<Request, Response> {
    request: channel::multi::MultiProducer<Request>,
    response: channel::unary::UnaryConsumer<Response>,
}

impl<Request, Response> ClientStreamingHandle<Request, Response> {
    /// Send a request to the server.
    ///
    /// # Parameters
    /// * `request` - The request to send.
    #[inline]
    pub async fn send(
        &mut self,
        request: Request,
    ) -> Result<(), channel::error::ProduceError> {
        self.request.send(request).await
    }

    /// Receive a response from the server, consuming [`self`].
    #[inline]
    pub async fn recv(self) -> Result<Response, channel::error::ConsumeError> {
        self.response
            .recv()
            .await
            .ok_or(channel::error::ConsumeError::Closed)
    }
}

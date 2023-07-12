use remi_util::channel;

/// A type to represent a request with an optionl response.
#[derive(Debug)]
pub struct UnaryRequest<Request, Response> {
    request: Request,
    response: Option<channel::unary::UnaryProducer<Response>>,
}

impl<Request, Response> UnaryRequest<Request, Response> {
    /// Create a new unary request with a required response.
    ///
    /// # Parameters
    /// * `request` - The request to send.
    #[inline]
    pub fn new(
        request: Request,
    ) -> (Self, channel::unary::UnaryConsumer<Response>) {
        let (tx, rx) = channel::unary::unary();

        let req = Self {
            request,
            response: Some(tx),
        };

        (req, rx)
    }

    /// Create a new unary request without a response.
    ///
    /// # Parameters
    /// * `request` - The request to send.
    #[inline]
    pub const fn new_detached(request: Request) -> Self {
        Self {
            request,
            response: None,
        }
    }

    /// Split the request and response sides of the request.
    #[inline]
    pub fn split(
        self,
    ) -> (Request, Option<channel::unary::UnaryProducer<Response>>) {
        (self.request, self.response)
    }
}

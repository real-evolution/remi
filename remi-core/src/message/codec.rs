/// A trait to be implemented by types that convert frames to messages.
pub trait Decoder<F> {
    type Error;
    type Item: super::Message;

    /// Deocdes a frame into a message.
    ///
    /// # Parameters
    /// - `frame`: The frame to decode.
    ///
    /// # Returns
    /// The decoded message.
    fn decode(&mut self, frame: F) -> Result<Option<Self::Item>, Self::Error>;
}

/// A trait to be implemented by types that convert messages to frames. The
/// opposite of [`MessageDecoder<F>`].
pub trait Encoder<F> {
    type Error;
    type Item: super::Message;

    /// Encodes a message into a frame.
    ///
    /// # Parameters
    /// - `message`: The message to encode.
    ///
    /// # Returns
    /// The encoded frame.
    fn encode(&mut self, message: Self::Item) -> Result<F, Self::Error>;
}

/// A convenience trait that combines [`MessageDecoder<F>`] and
/// [`MessageEncoder<F>`].
pub trait Codec<F>: Decoder<F> + Encoder<F> {}

impl<F, T> Codec<F> for T where T: Decoder<F> + Encoder<F> {}

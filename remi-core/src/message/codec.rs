/// A trait to be implemented by types that convert frames to messages.
pub trait MessageDecoder<F> {
    type Error;
    type Metadata;
    type Data;

    /// Deocdes a frame into a message.
    ///
    /// # Parameters
    /// - `frame`: The frame to decode.
    ///
    /// # Returns
    /// The decoded message.
    fn decode(
        &mut self,
        frame: F,
    ) -> Result<super::Message<Self::Metadata, Self::Data>, Self::Error>;
}

/// A trait to be implemented by types that convert messages to frames. The
/// opposite of [`MessageDecoder<F>`].
pub trait MessageEncoder<F> {
    type Error;
    type Metadata;
    type Data;

    /// Encodes a message into a frame.
    ///
    /// # Parameters
    /// - `message`: The message to encode.
    ///
    /// # Returns
    /// The encoded frame.
    fn encode(
        &mut self,
        message: super::Message<Self::Metadata, Self::Data>,
    ) -> Result<F, Self::Error>;
}

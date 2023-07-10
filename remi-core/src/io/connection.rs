/// A trait to represent a transport connection.
#[crate::async_trait]
pub trait Connection {
    type Frame: Send;
    type Error;

    /// Sends a frame through the connection.
    async fn send(&mut self, frame: Self::Frame) -> Result<(), Self::Error>;

    /// Receives a frame from the connection.
    async fn next(&mut self) -> Option<Result<Self::Frame, Self::Error>>;
}

/// A trait to represent an item with an address.
pub trait Addressable {
    type Address;

    /// Returns the address of this item.
    fn address(&self) -> &Self::Address;
}

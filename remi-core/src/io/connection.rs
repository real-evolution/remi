use futures::{Sink, Stream};

/// A trait to represent a transport connection.
pub trait Connection<Frame>: Sink<Frame> + Stream<Item = Frame> {}

/// A trait to represent an item with an address.
pub trait Addressable {
    type Address;

    /// Returns the address of this item.
    fn address(&self) -> &Self::Address;
}

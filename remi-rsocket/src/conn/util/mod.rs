/// Utilities for types that implement [`Sink`](futures::sink::Sink).
mod sink;

#[doc(inline)]
pub use sink::SinkErrorAdapter;

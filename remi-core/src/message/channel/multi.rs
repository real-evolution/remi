/// A multi-producer, multi-consumer sender side.
#[derive(Debug)]
pub struct MultiProducer<T>(kanal::AsyncSender<T>);

/// A multi-producer, multi-consumer consumer side.
#[derive(Debug)]
pub struct MultiConsumer<T>(kanal::AsyncReceiver<T>);

/// Creates an unbounded multi-producer, multi-consumer channel.
#[inline]
#[must_use]
pub fn unbounded<T>() -> (MultiProducer<T>, MultiConsumer<T>) {
    let (sender, receiver) = kanal::unbounded_async();

    (MultiProducer(sender), MultiConsumer(receiver))
}

/// Creates a bounded multi-producer, multi-consumer channel.
///
/// # Parameters
/// * `bound` - The maximum number of elements the channel can hold.
#[inline]
#[must_use]
pub fn bounded<T>(bound: usize) -> (MultiProducer<T>, MultiConsumer<T>) {
    let (sender, receiver) = kanal::bounded_async(bound);

    (MultiProducer(sender), MultiConsumer(receiver))
}

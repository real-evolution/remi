/// A oneshot channel sender side.
#[derive(Debug)]
pub struct UnaryProducer<T>(kanal::OneshotAsyncSender<T>);

/// A oneshot channel consumer side.
#[derive(Debug)]
pub struct UnaryConsumer<T>(kanal::OneshotAsyncReceiver<T>);

#[inline]
#[must_use]
pub fn unary<T>() -> (UnaryProducer<T>, UnaryConsumer<T>) {
    let (sender, receiver) = kanal::oneshot_async();

    (UnaryProducer(sender), UnaryConsumer(receiver))
}

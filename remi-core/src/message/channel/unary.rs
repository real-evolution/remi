/// A oneshot channel sender side.
#[derive(Debug)]
pub struct UnaryProducer<T>(kanal::OneshotAsyncSender<T>);

impl<T> UnaryProducer<T> {
    /// Sends a value to the consumer, consuming this sender.
    ///
    /// The value is returned back if the value could not be sent.
    ///
    /// # Parameters
    /// - `value`: The value to send.
    #[inline]
    pub async fn send(self, value: T) -> Result<(), T> {
        self.0.send(value).await
    }
}

/// A oneshot channel consumer side.
#[derive(Debug)]
pub struct UnaryConsumer<T>(kanal::OneshotAsyncReceiver<T>);

impl<T> UnaryConsumer<T> {
    /// Receives a value from the producer.
    ///
    /// # Returns
    /// The received value.
    #[inline]
    pub async fn recv(self) -> Option<T> {
        self.0.recv().await.ok()
    }
}

#[inline]
#[must_use]
pub fn unary<T>() -> (UnaryProducer<T>, UnaryConsumer<T>) {
    let (sender, receiver) = kanal::oneshot_async();

    (UnaryProducer(sender), UnaryConsumer(receiver))
}

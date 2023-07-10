mod accept;

use remi_core::io::Acceptor;

pub trait AcceptorExt: Sized {
    /// Asynchrnously accepts a connection.
    fn accept(&mut self) -> accept::Accept<'_, Self> {
        accept::Accept::new(self)
    }
}

impl<A: Acceptor> AcceptorExt for A {}

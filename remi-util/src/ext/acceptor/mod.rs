mod accept;

use remi_core::edge::Acceptor;

pub trait AcceptorExt: Acceptor + Sized {
    /// Asynchrnously accepts a connection.
    fn accept(&mut self) -> accept::Accept<'_, Self> {
        accept::Accept::new(self)
    }
}

impl<A: Acceptor> AcceptorExt for A {}

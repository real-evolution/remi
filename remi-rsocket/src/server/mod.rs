mod service;

pub use service::RSocketServer;

use crate::io::Connection;

#[derive(Debug)]
pub struct RSocket<Conn> {
    _conn: Connection<Conn>,
}

impl<Conn> RSocket<Conn> {
    #[inline]
    pub fn new(conn: Connection<Conn>) -> Self {
        Self { _conn: conn }
    }
}

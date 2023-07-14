mod io;
mod service;

pub(crate) use io::Connection;

#[derive(Debug)]
pub struct RSocket<Conn> {
    _conn: io::Connection<Conn>,
}

impl<Conn> RSocket<Conn> {
    #[inline]
    pub fn new(conn: Connection<Conn>) -> Self {
        Self { _conn: conn }
    }
}

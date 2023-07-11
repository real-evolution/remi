use remi_core::io::Acceptor;
use remi_util::ext::AcceptorExt;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};

pub async fn serve<A>(acceptor: A) -> std::io::Result<()>
where
    A: Acceptor,
    A::Address: std::fmt::Debug,
    A::Connection: AsyncRead + AsyncWrite,
    std::io::Error: From<A::Error>,
    <A as remi_core::io::Acceptor>::Connection: std::marker::Unpin,
{
    let mut acceptor = acceptor;

    loop {
        let (mut conn, addr) = acceptor.accept().await?.split();

        println!("Accepted connection from {:?}", addr);

        let mut buf = [0u8; 1024];
        let n = conn.read(&mut buf).await?;
        let s = std::str::from_utf8(&buf[..n]).unwrap();

        println!("Received: {}", s);
    }
}

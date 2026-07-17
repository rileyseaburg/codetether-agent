//! Reusable UDP socket construction for same-host discovery peers.

use socket2::{Domain, Protocol, Socket, Type};
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::UdpSocket;

pub(super) fn bind() -> std::io::Result<UdpSocket> {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
    socket.set_reuse_address(true)?;
    socket.set_broadcast(true)?;
    socket.set_nonblocking(true)?;
    socket.bind(&SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, super::beacon::PORT).into())?;
    UdpSocket::from_std(socket.into())
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn multiple_listeners_can_share_the_discovery_port() {
        let first = super::bind().expect("first listener");
        let second = super::bind().expect("second listener");
        assert_eq!(first.local_addr().unwrap(), second.local_addr().unwrap());
    }
}

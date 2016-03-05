extern crate mio_websocket;

use std::net::SocketAddr;
use mio_websocket::interface::*;

fn main() {
    let mut ws = WebSocket::new("127.0.0.1:8080".parse::<SocketAddr>().unwrap());

    loop {
        match ws.next() {
            event @ (_, WebSocketEvent::TextMessage(_)) |
            event @ (_, WebSocketEvent::BinaryMessage(_)) => {
                ws.send(event);
            }
            _ => {}
        }
    }
}

extern crate mio_websocket;

use std::net::SocketAddr;
use mio_websocket::interface::*;

fn main() {
    let mut ws = WebSocket::new("127.0.0.1:2794".parse::<SocketAddr>().unwrap());

    println!("Listening!");

    loop {
        match ws.next() {
            (tok, WebSocketEvent::TextMessage(msg)) => {
                match ws.get_connected() {
                    Ok(val) => {
                        for token in val {
                            let response = WebSocketEvent::TextMessage(msg.clone());
                            ws.send((token, response));
                        }
                    }
                    Err(e) => {
                        panic!(e);
                    }
                }
            }
            _ => {}
        }
    }
}

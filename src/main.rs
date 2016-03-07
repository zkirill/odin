extern crate ws;

use ws::{listen, Handler, Sender, Result, Message, CloseCode};

struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Handle new message.

        // Broadcast the message to all connections.
        self.out.broadcast(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        // Handle connection close.
        match code {
            CloseCode::Normal => println!("Connection closed."),
            CloseCode::Away => println!("Connection away."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

fn main() {
    let addr = "127.0.0.1:2794";
    println!("Listening on {:?}...", addr);
    listen(addr, |out| Server { out: out }).unwrap()
}

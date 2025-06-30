use mio::{Events, Interest, Poll, Token, net::TcpListener};
use std::{net::SocketAddr, time::Duration};

fn main() -> std::io::Result<()> {
    // 1. Create a Poll instance (replaces `EventLoop`)
    let mut poll = Poll::new()?;

    // 2. Create storage for events (replaces `Handler`)
    let mut events = Events::with_capacity(128);

    let address = "0.0.0.0:1000".parse::<SocketAddr>().unwrap();
    let mut server_socket = TcpListener::bind(address).unwrap();
    poll.registry()
        .register(&mut server_socket, Token(0), Interest::READABLE)?;
    // 3. Main event loop (replaces `event_loop.run()`)
    loop {
        // Wait for events (blocking)
        poll.poll(&mut events, None)?;

        // Process events
        for event in events.iter() {
            match event.token() {
                Token(0) => match server_socket.accept() {
                    Ok((connection, address)) => {
                        println!("accepted conectio on {}", address)
                    }
                    Err(e) => println!("accept error:{} ", e),
                },
                _ => println!("Unknown event"),
            }
        }
    }
}

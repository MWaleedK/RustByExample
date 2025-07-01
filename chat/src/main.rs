use mio::{
    Events, Interest, Poll, Token,
    net::{TcpListener, TcpStream},
};
use std::collections::HashMap;
use std::net::SocketAddr;

struct WebSocketServer {
    socket: TcpListener,
    clients: HashMap<Token, TcpStream>,
    token_counter: usize,
}

impl WebSocketServer {
    fn new(address: SocketAddr) -> std::io::Result<Self> {
        let socket = TcpListener::bind(address)?;
        Ok(Self {
            socket,
            clients: HashMap::new(),
            token_counter: 1, // Start from 1 (0 is reserved for the server socket)
        })
    }

    fn run(&mut self) -> std::io::Result<()> {
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(128);

        // Register the server socket
        poll.registry()
            .register(&mut self.socket, Token(0), Interest::READABLE)?;

        println!("Server listening...");

        loop {
            // Wait for events (blocking)
            poll.poll(&mut events, None)?;

            for event in events.iter() {
                match event.token() {
                    Token(0) => {
                        // Accept new connections
                        loop {
                            match self.socket.accept() {
                                Ok((mut connection, address)) => {
                                    println!("Accepted connection from {}", address);

                                    let token = Token(self.token_counter);
                                    self.token_counter += 1;

                                    // Register the new client socket
                                    poll.registry().register(
                                        &mut connection,
                                        token,
                                        Interest::READABLE,
                                    )?;

                                    // Store client
                                    self.clients.insert(token, connection);
                                }
                                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                    // No more connections to accept
                                    break;
                                }
                                Err(e) => {
                                    eprintln!("Accept error: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                    token => {
                        // Handle client events (read readiness)
                        if let Some(stream) = self.clients.get_mut(&token) {
                            let mut buf = [0; 1024];
                            match stream.read(&mut buf) {
                                Ok(0) => {
                                    // Connection closed
                                    println!("Client {:?} disconnected", token);
                                    self.clients.remove(&token);
                                }
                                Ok(n) => {
                                    println!(
                                        "Read {} bytes from client {:?}: {:?}",
                                        n,
                                        token,
                                        &buf[..n]
                                    );
                                    // You could write a response here if needed
                                }
                                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                    // No data to read
                                }
                                Err(e) => {
                                    eprintln!("Read error: {}", e);
                                    self.clients.remove(&token);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

use std::io::Read;

fn main() -> std::io::Result<()> {
    let address = "0.0.0.0:1000".parse::<SocketAddr>().unwrap();
    let mut server = WebSocketServer::new(address)?;
    server.run()
}

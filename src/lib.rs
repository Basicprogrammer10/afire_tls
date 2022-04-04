use std::io::{Read, Write};
use std::sync::Arc;

use afire::{internal::socket_handler::SocketHandler, Server};
pub use native_tls::Identity;
use native_tls::TlsAcceptor;

pub struct AfireTls {
    acceptor: Arc<TlsAcceptor>,
}

impl AfireTls {
    pub fn new(identity: Identity) -> Self {
        let acceptor = TlsAcceptor::new(identity).unwrap();
        let acceptor = Arc::new(acceptor);

        Self { acceptor }
    }

    pub fn attatch(self, server: &mut Server) {
        let ac1 = self.acceptor.clone();
        let ac2 = self.acceptor.clone();
        let ac3 = self.acceptor.clone();
        let ac4 = self.acceptor;

        server.socket_handler = SocketHandler {
            socket_read: Box::new(move |x, buff| ac1.accept(x).ok()?.read(buff).ok()),
            socket_read_exact: Box::new(move |x, buff| ac2.accept(x).ok()?.read_exact(buff).ok()),
            socket_flush: Box::new(move |x| ac3.accept(x).ok()?.flush().ok()),
            socket_write: Box::new(move |x, y| ac4.accept(x).unwrap().write_all(y).ok()),
        };
    }
}
